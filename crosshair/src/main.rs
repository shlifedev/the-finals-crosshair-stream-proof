#![allow(dead_code)]
#![feature(const_fn_floating_point_arithmetic)]
use sysinfo::{System, SystemExt,PidExt,UserExt, ProcessExt};
use std::{
    cell::{
        RefCell,
        RefMut,
    },
    error::Error,
    fmt::Debug,
    fs::File,
    io::BufWriter,
    path::PathBuf,
    rc::Rc,
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
    },
    time::{
        Duration,
        Instant,
    }, thread::{sleep, sleep_ms}, ops::Div,
};

use anyhow::{Context, Ok};
use clap::{
    Args,
    Parser,
    Subcommand,
};
use imgui::{
    Condition,
    FontConfig,
    FontId,
    FontSource,
    Ui, sys::{ImVec2_ImVec2_Float, ImColor_ImColor_Float, ImColor, ImVec4},
};

use overlay::{
    LoadingError,
    OverlayError,
    OverlayOptions,
    OverlayTarget,
    SystemRuntimeController,
};

enum AppCommand {
    /// Start the overlay
    Overlay,

    /// Create a schema dump
    DumpSchema(SchemaDumpArgs),
}


pub struct AppFonts {
    fid: FontId,
}

use obfstr::obfstr;

use native_dialog::{FileDialog, MessageDialog, MessageType};

pub struct Application {

}
fn drawCross(ui: &imgui::Ui, distance : f32, thickness : f32) {

 
    let mut x = ui.io().display_size[0].div(2.0);
    let mut y =ui.io().display_size[1].div(2.0);
    let mut length = 6.0; 
     
    let chColor = [0.0, 255.0, 0.0];
    ui.get_window_draw_list().add_line([x-distance, y], [x-distance-length, y], chColor).thickness(thickness).build(); 
    ui.get_window_draw_list().add_line([x+distance, y], [x+distance+length, y], chColor).thickness(thickness).build();
    
    ui.get_window_draw_list().add_line([x ,y-distance], [x, y-distance-length], chColor).thickness(thickness).build();
    ui.get_window_draw_list().add_line([x, y+distance], [x, y+distance+length], chColor).thickness(thickness).build(); 
} 


fn drawDot(ui: &imgui::Ui, size : f32) { 
    let mut x = ui.io().display_size[0].div(2.0);
    let mut y =ui.io().display_size[1].div(2.0);
    ui.get_window_draw_list().add_circle([x,y], 3.0, [0.0,0.0, 0.0]).build();
    ui.get_window_draw_list().add_circle([x,y], 2.0, [0.0,255.0, 0.0]).build();
} 
    
 

impl Application {

     
    pub fn pre_update(&mut self, controller: &mut SystemRuntimeController) -> anyhow::Result<()> {
      

      
      controller.toggle_screen_capture_visibility(false);
        Ok(())
    }

    pub fn update(&mut self, ui: &imgui::Ui) -> anyhow::Result<()> {
        {

        } 


        Ok(())
    }

 
    fn render_overlay(&self, ui: &imgui::Ui) {

        
        let text_buf;
        let text = obfstr!(text_buf = "Overlay");
        
        // 윈도우 포지션
        let mut x = ui.io().display_size[0].div(2.0);
        let mut y =ui.io().display_size[1].div(2.0);

        // 크로스헤어 크기 
        {
        //    drawDot(&ui, 3.0);
        }

        {

            if ui.io().mouse_down[1] == true {
                drawCross(&ui, 5.0, 2.0);
            }
            else{
                drawCross(&ui, 10.0, 3.0);
            } 
        }
        ui.text("Crosshair Overlay");
    }
    
 
    pub fn render(&self, ui: &imgui::Ui) {
        ui.window("overlay")
        .draw_background(false)
        .no_decoration()
        .no_inputs()
        .size(ui.io().display_size, Condition::Always)
        .position([0.0, 0.0], Condition::Always)
        .build(|| self.render_overlay(ui));
  
 
    }


}

struct SchemaDumpArgs {
    pub target_file: PathBuf,
}


fn main(){
  

 
    let cmd : Option<AppCommand> = Default::default();
    let command = cmd.as_ref().unwrap_or(&AppCommand::Overlay);
    let result = match command {
        AppCommand::DumpSchema(args) => {},
        AppCommand::Overlay => main_overlay().expect("")
    };
 
    

 
}

fn main_overlay() -> anyhow::Result<()> {


   
let s = System::new_all();
let mut pid = 0;
for process in s.processes_by_name("Discovery.exe") {
    pid = process.pid().as_u32(); 
}

/*
THE FINALS  
ahk_class DefensiveBat_cc564fbe
ahk_exe Discovery.exe
ahk_pid 25776
ahk_id 1316060
 */
    let title = String::from("Discovery.exe");
    let app_fonts: Rc<RefCell<Option<AppFonts>>> = Default::default();
    let overlay_options = OverlayOptions {
        title: obfstr!("FN Overlay").to_string(),
        target: OverlayTarget::WindowOfProcess(pid),
        font_init: Some(Box::new({
            let app_fonts = app_fonts.clone();

            move |imgui| {
                let mut app_fonts = app_fonts.borrow_mut();

                let font_size = 18.0;
                let font = imgui.fonts().add_font(&[FontSource::TtfData {
                    data: include_bytes!("../resources/Font.ttf"),
                    size_pixels: font_size,
                    config: Some(FontConfig {
                        rasterizer_multiply: 1.5,
                        oversample_h: 4,
                        oversample_v: 4,
                        ..FontConfig::default()
                    }),
                }]);

                *app_fonts = Some(AppFonts {
                    fid: font,
                });
            }
        })),
    };

    let app = Application{

    };
    let app = Rc::new(RefCell::new(app));
    let overlay = match overlay::init(&overlay_options) {
        value => value?,
    };


    let mut update_fail_count = 0;
    let mut update_timeout: Option<(Instant, Duration)> = None;


 overlay.main_loop({
    let app = app.clone();
                       move |controller| {
                           let mut app = app.borrow_mut();
                           if let Err(_err) = app.pre_update(controller) {
                               false
                           } else {
                               true
                           }
                       }
 }, move |ui| {
     let mut app = app.borrow_mut();


     if let Some((timeout, target)) = &update_timeout {
        if timeout.elapsed() > *target {
            update_timeout = None;
        } else {
            /* Not updating. On timeout... */
            return true;
        }
    }



     if let Err(err) = app.update(ui) {
         if update_fail_count >= 10 {
             log::error!("Over 10 errors occurred. Waiting 1s and try again.");
             log::error!("Last error: {:#}", err);

             update_timeout = Some((Instant::now(), Duration::from_millis(1000)));
             update_fail_count = 0;
             return true;
         } else {
             update_fail_count += 1;
         }
     }

     app.render(ui);
     true
 }
 );
}