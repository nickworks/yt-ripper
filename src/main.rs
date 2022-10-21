use text_io::read;
use rustube::{Id, VideoFetcher, Callback};
use terminal_menu::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    
    println!(" ");
    println!("  --------------------------------------------------------------------------");
    print!("   youtube url > ");
    
    let input: String = read!("{}\n");
    let url:&str = &input;

    let vid_id = Id::from_raw(url);
    
    if let Err(_) = vid_id {
        println!("  --------------------------------------------------------------------------");
        println!("   video not found");
        return;
    }

    let descrambler = VideoFetcher::from_id(vid_id.unwrap().into_owned()).unwrap().fetch().await.unwrap();

    let title = descrambler.video_title().clone();
    
    let mut menu_items = vec![
        label("--------------------------------------------------------------------------"),
        label(format!(" {: ^72} ",&title)),
        label("--------------------------------------------------------------------------"),
        label("  resolution    bitrate    audio hz  audio video    codecs"),
        label("-------------|-----------|----------|-----|-----|-------------------------"),
    ];

    let index_offset = menu_items.len();

    let vid = descrambler.descramble().unwrap();
    for stream in vid.streams() {
        let c:Vec<&str> = stream.codecs.iter().map(|s|s.as_str()).collect();
        let resolution = format!(
            "{: >5} x {: <5}",
            stream.width.unwrap_or(0),
            stream.height.unwrap_or(0),
        );
        menu_items.push(button(format!(
            "{: ^13} {: >10} {: >9}   {: ^5} {: ^5}  {: <20}",
            match stream.includes_video_track {
                true => resolution.as_str(),
                false => "-",
            },
            stream.bitrate.unwrap_or(0),
            stream.audio_sample_rate.unwrap_or(0),
            match stream.includes_audio_track {
                true => "Y",
                false => "-",
            },
            match stream.includes_video_track {
                true => "Y",
                false => "-",
            },
            c.join(", ").to_string(),
        )));
    }
    menu_items.push(button("cancel"));

    let choice = menu(menu_items);
    run(&choice);

    let choice = mut_menu(&choice).selected_item_index() - index_offset;
    let choice = vid.streams().get(choice);

    if let Some(stream) = choice {

        let callback = Callback::new().connect_on_progress_closure(|p|{
            if let Some(content_length) = p.content_length {
                println!("    download {}%", (p.current_chunk * 100) as f64 / content_length as f64);
            }
        });
        println!("  --------------------------------------------------------------------------");
        println!("   Downloading {} ...", &title);
        println!("  --------------------------------------------------------------------------");
        let path = stream.download_with_callback(callback).await.unwrap();
        println!("  --------------------------------------------------------------------------");
        println!("   Downloaded to {}", path.to_str().unwrap_or("-"));
        println!("  --------------------------------------------------------------------------");
    }
    println!("");
}

