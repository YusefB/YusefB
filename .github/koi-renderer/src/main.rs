use std::{env, error::Error, fs, path::PathBuf};

use git_koi::{
    behavior::Behavior,
    coat::KoiBody,
    history::AccountSnapshot,
    koi::derive_koi_development,
    motion::frame_count,
    output::encode_gif,
    render::render_swim_frame,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let snapshot_path = args.next().ok_or("missing snapshot path")?;
    let output_path = PathBuf::from(args.next().ok_or("missing output path")?);
    if args.next().is_some() {
        return Err("expected snapshot and output paths".into());
    }
    let snapshot: AccountSnapshot = serde_json::from_slice(&fs::read(snapshot_path)?)?;
    let koi = derive_koi_development(&snapshot)?;
    // Preserve the real account's appearance, but always swim a clockwise lap.
    let behavior = Behavior::Cruise;
    let frames: Vec<_> = (0..frame_count(behavior))
        .map(|frame| render_swim_frame(&koi, KoiBody::Black, behavior, frame))
        .collect();
    let gif = encode_gif(&frames)?;
    let temporary_path = output_path.with_extension("tmp");
    fs::write(&temporary_path, gif)?;
    fs::rename(temporary_path, output_path)?;
    println!("Rendered black koi in cruise mode ({} frames)", frames.len());
    Ok(())
}
