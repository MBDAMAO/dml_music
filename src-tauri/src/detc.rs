use std::sync::{Arc, Mutex as TMUTX};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use pitch_detection::{detector::mcleod::McLeodDetector, detector::PitchDetector};
use ringbuf::traits::{Consumer, Producer, Split};
use ringbuf::HeapRb;
use tauri::State;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, Mutex};
const SAMPLE_RATE: usize = 44100;
// 添加命令枚举
#[derive(Debug)]
pub enum StreamCommand {
    Stop,
}

pub struct AppState {
    pub command_sender: Mutex<Option<mpsc::Sender<StreamCommand>>>,
}

#[tauri::command]
pub async fn start_pitch_detection(app: AppHandle) -> Result<(), String> {
    const SIZE: usize = 1024;

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to find a default input device");

    println!("Using device: {}", device.name().unwrap());

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");

    let buffer: Arc<TMUTX<Vec<f32>>> = Arc::new(TMUTX::new(Vec::new()));
    let buffer_clone: Arc<TMUTX<Vec<f32>>> = buffer.clone();

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buffer = buffer_clone.lock().unwrap();
                buffer.extend_from_slice(data);

                while buffer.len() >= SIZE {
                    let chunk = buffer.drain(0..SIZE).collect::<Vec<f32>>();
                    let (freq, note) = process_chunk(&chunk);
                    let _ = app.emit("pitch_data", freq);
                }
            },
            |error| eprintln!("An error occurred on stream: {}", error),
            None,
        )
        .expect("Failed to build input audio stream");

    stream.play().expect("Failed to play stream");

    loop {}
    Ok(())
}

fn process_chunk(chunk: &[f32]) -> (std::string::String, &str) {
    const A4_FREQUENCY: f64 = 440.0; // A4 的标准频率
    const SEMITONE_RATIO: f64 = 1.0594630943592953; // 2^(1/12)

    let mut detector = McLeodDetector::new(chunk.len(), chunk.len() / 2);
    if let Some(pitch) = detector.get_pitch(chunk, SAMPLE_RATE, 0.1, 0.7) {
        let frequency = pitch.frequency as f64;

        // 计算与 A4 的半音差
        let semitones_from_a4 = 12.0 * (frequency / A4_FREQUENCY).log2();

        // 计算音名和八度
        let mut note_index = semitones_from_a4.round() as i32;
        let octave = 4 + (note_index / 12);
        if note_index < 0 {
            note_index = -note_index;
        }
        let note_name = match note_index % 12 {
            0 => "A",
            1 => "A#",
            2 => "B",
            3 => "C",
            4 => "C#",
            5 => "D",
            6 => "D#",
            7 => "E",
            8 => "F",
            9 => "F#",
            10 => "G",
            11 => "G#",
            _ => unreachable!(),
        };

        println!(
            "Frequency: {:.2} Hz, Note: {}{}, Clarity: {:.2}",
            frequency, note_name, octave, pitch.clarity
        );
        return (frequency.to_string(), note_name);
    } else {
        return ("0".to_owned(), "N");
    }
}

#[tauri::command]
pub async fn stop_pitch_detection(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(sender) = state.command_sender.lock().await.take() {
        sender
            .send(StreamCommand::Stop)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
