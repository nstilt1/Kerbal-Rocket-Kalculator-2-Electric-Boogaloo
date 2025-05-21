#![feature(const_trait_impl)]

use std::{collections::HashSet, fmt::Display};

#[cfg(target_arch = "wasm32")]
use js_sys::Array;
#[allow(unused)]
use modules::{calculator::Calculator};
use wasm_bindgen::prelude::*;

pub mod modules;

const G: f64 = 9.80665;

#[derive(Debug)]
pub enum Error {
    
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str("Error")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            _ => "Error"
        };
        f.write_str(&s)
    }
}

impl From<Error> for JsError {
    fn from(value: Error) -> Self {
        Self::new(&value.to_string())
    }
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn get_chords_of_key(
    mut key: &str,
    chord_selection: Array,
    chord_type_group: &str,
    scale: &str,
    table_scheme: &str,
    show_probabilities: bool,
) -> Result<String, JsError> {
    use serde_json::json;

    let use_all_roots = key.eq("random");
    if use_all_roots {
        key = "Cmin";
    }
    let chord_selection_hashset: HashSet<String> = chord_selection.iter()
        .map(|js_val| js_val.as_string().unwrap_or_default())
        .collect();
    let mut musician = Music::smoke_hash(Default::default(), "Cmin", &chord_selection_hashset, chord_type_group, scale, false, use_all_roots)?;

    match table_scheme {
        "contains_note" => musician.rotate_chords(key),
        "highest_note" => musician.rearrange_by_highest_note(key),
        "lowest_note" => musician.rearrange_by_lowest_note(key),
        _ => return Err(JsError::new("table_scheme did not match"))
    }

    if show_probabilities {
        musician.set_probabilities();
    }

    // sort the sub-arrays
    for col in musician.chord_table.iter_mut() {
        col.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));
    }

    musician.chord_list.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));

    let json = json!({
        "chord_list": musician.chord_list,
        "chord_table": musician.chord_table
    });

    //let json = to_string(&musician.chord_table)?;
    
    Ok(json.to_string())
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn chord_finder(
    mut key: &str,
    chord_selection: Array,
    chord_type_group: &str,
    scale: &str,
    notes: Array,
    table_scheme: &str
) -> Result<String, JsError> {
    use music_modules_v2::{chord::Chord, utils::{parse_key, sets::{SetMath, SetOpsCollection}}};
    use serde_json::json;
    let use_all_roots = key.eq("random");
    if use_all_roots {
        key = "Cmin";
    }
    let chord_selection_hashset: HashSet<String> = chord_selection.iter()
        .map(|js_val| js_val.as_string().unwrap_or_default())
        .collect();
    let notes_vec: Vec<usize> = notes
        .iter()
        .map(|js_val| parse_key(&js_val.as_string().unwrap_or_default()) as usize)
        .collect();
    if notes_vec.is_empty() {
        return Ok(json!({}).to_string())
    }
    let mut musician = Music::smoke_hash(Default::default(), "Cmin", &chord_selection_hashset, chord_type_group, scale, false, use_all_roots)?;

    musician.rotate_chords(key);

    let mut intersected_chords: HashSet<Chord> = HashSet::from_iter(musician.chord_table[notes_vec[0]].iter().cloned());
    for note in notes_vec.iter().skip(1) {
        intersected_chords = intersected_chords
            .intersection(
                &HashSet::from_iter(musician.chord_table[*note % 12]
                    .iter()
                    .cloned()
                )
            ).to_set();
    }

    match table_scheme {
        "contains_note" => (),
        "highest_note" => musician.rearrange_by_highest_note(key),
        "lowest_note" => musician.rearrange_by_lowest_note(key),
        _ => return Err(JsError::new("table_scheme did not match"))
    }

    for chords in musician.chord_table.iter_mut() {
        *chords = HashSet::from_iter(chords.iter().cloned())
            .intersection(&intersected_chords)
            .to_vec();
    }
    musician.chord_list = intersected_chords.to_vec();

    // sort the sub-arrays
    for col in musician.chord_table.iter_mut() {
        col.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));
    }

    musician.chord_list.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));

    let json = json!({
        "chord_list": musician.chord_list,
        "chord_table": musician.chord_table
    });

    Ok(json.to_string())
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn generate_midi(
    file_content: &[u8], 
    generation_mode: &str, 
    should_use_same_chords: bool, 
    num_chords: usize, 
    key: &str,
    chord_selection: Array,
    chord_type_group: &str,
    chord_picking_method: &str,
    min_number_of_unique_chords: u32,
    scale: &str,
    is_reproducible: bool,
) -> Result<Vec<u8>, Error> {
    let hash = Sha256::digest(file_content);
    
    let chord_selection_hashset: HashSet<String> = chord_selection.iter()
        .map(|js_val| js_val.as_string().unwrap_or_default())
        .collect();
    // smoke the hash
    let mut musician = Music::smoke_hash(hash, key, &chord_selection_hashset, chord_type_group, scale, is_reproducible, false)?;
    let track = musician.make_music(num_chords, generation_mode, should_use_same_chords, chord_picking_method, min_number_of_unique_chords)?;

    let smf = Smf {
        header: midly::Header { format: midly::Format::SingleTrack, timing: midly::Timing::Metrical(96.into()) },
        tracks: vec![track]
    };

    let mut output = vec![];

    smf.write(&mut output)?;

    Ok(output)
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn generate_midi_chord_progression(chords: JsValue) -> Result<Vec<u8>, JsError> {
    let chords: Vec<Vec<usize>> = serde_wasm_bindgen::from_value(chords).expect("Bad input");
    
    let mut track = MidiFile::new();
    let mut time = 0f64;
    for chord in chords.iter() {
        for note in chord {
            track.add_note_beats(*note as u8, time, 4f64, 80);
        }
        time += 4f64;
    }
    let track = track.finalize();
    let smf = Smf {
        header: midly::Header { format: midly::Format::SingleTrack, timing: midly::Timing::Metrical(96.into()) },
        tracks: vec![track]
    };
    let mut output = vec![];
    smf.write(&mut output).expect("Should be valid");
    Ok(output)
}

#[cfg(not(target_arch = "wasm32"))]
pub mod test_utils {

}

#[cfg(test)]
mod tests {
    use super::*;

    
}