use minreq::get;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub struct VoicyTTS;

#[derive(Clone, Copy)]
pub enum Language {
    English,
    Russian,
}

#[derive(Clone)]
pub struct VoicyTTSOptions {
    text: String,
    filename: String,
    language: Language,
}

#[allow(dead_code)]
impl VoicyTTSOptions {
    pub fn default() -> Self {
        Self {
            text: "Hello, World!".to_string(),
            filename: "VTTS.mp3".to_string(),
            language: Language::English,
        }
    }

    pub fn as_link(&self) -> String {
        format!(
            "https://translate.google.de/translate_tts?ie=UTF-8&q={}&tl={}&total=1&idx=0&textlen={}&tl={}&client=tw-ob",
            utf8_percent_encode(self.text.as_str(), FRAGMENT).to_string(),
            self.language.to_string(),
            self.text.len(),
            self.language.to_string()
        )
    }

    pub fn set_text(&mut self, text: String) -> &Self {
        self.text = text;
        self
    }

    pub fn set_filename(&mut self, filename: String) -> &Self {
        self.filename = filename;
        self
    }

    pub fn set_language(&mut self, language: Language) -> &Self {
        self.language = language;
        self
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn get_language(&self) -> &Language {
        &self.language
    }
}

impl Language {
    pub fn to_string(self) -> String {
        match self {
            Language::English => String::from("en"),
            Language::Russian => String::from("ru"),
        }
    }
}

impl VoicyTTS {
    pub fn to_file(options: &VoicyTTSOptions) -> Result<(), std::io::Error> {
        if let Ok(response) = get(options.clone().as_link()).send() {
            if let Ok(mut file) = File::create(options.filename.clone()) {
                let bytes = response.as_bytes();

                if bytes.len() > 0 {
                    let _ = file.write_all(bytes);
                }
            }
        }

        Ok(())
    }
}

pub struct VoicyTools;

impl VoicyTools {
    pub fn speed_up(in_file: &str, out_file: &str, new_speed: f32) {
        let status = Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(in_file)
            .arg("-filter:a")
            .arg(format!("atempo={}", new_speed))
            .arg(out_file)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to execute FFmpeg command");

        if !status.success() {
            panic!("Error during audio speedup")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_vtts_to_file() {
            let options = VoicyTTSOptions {
                text: "Test text".to_string(),
                filename: "test.mp3".to_string(),
                language: Language::English,
            };

            let _ = fs::remove_file(&options.filename);

            let result = VoicyTTS::to_file(&options);

            assert!(result.is_ok());

            assert!(fs::metadata(&options.filename).is_ok());

            let _ = fs::remove_file(&options.filename);
        }
    }
}
