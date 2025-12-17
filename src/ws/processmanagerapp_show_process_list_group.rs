use crate::ws::SortColumn;
use crate::ws::ProcessInfo;
use crate::ws::ViewMode;
use crate::ws::CustomProgram;
// # ProcessManagerApp - show_process_list_group Methods
//
// This module contains method implementations for `ProcessManagerApp`.
//
// 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn show_process_list(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(
                ui,
                |ui| {
                    ui.horizontal(|ui| {
                        ui.style_mut().spacing.item_spacing.x = 10.0;
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Pid,
                                format!(
                                    "PID {}", if self.sort_column == SortColumn::Pid { if self
                                    .sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Pid {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Pid;
                                self.sort_ascending = true;
                            }
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Name,
                                format!(
                                    "Name {}", if self.sort_column == SortColumn::Name { if self
                                    .sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Name {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Name;
                                self.sort_ascending = true;
                            }
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Memory,
                                format!(
                                    "Memory {}", if self.sort_column == SortColumn::Memory { if
                                    self.sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Memory {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Memory;
                                self.sort_ascending = false;
                            }
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Cpu,
                                format!(
                                    "CPU % {}", if self.sort_column == SortColumn::Cpu { if self
                                    .sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Cpu {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Cpu;
                                self.sort_ascending = false;
                            }
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::ParentPid,
                                format!(
                                    "Parent {}", if self.sort_column == SortColumn::ParentPid {
                                    if self.sort_ascending { "▲" } else { "▼" } } else { ""
                                    }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::ParentPid {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::ParentPid;
                                self.sort_ascending = true;
                            }
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Status,
                                format!(
                                    "Status {}", if self.sort_column == SortColumn::Status { if
                                    self.sort_ascending { "▲" } else { "▼" } } else { "" }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Status {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Status;
                                self.sort_ascending = true;
                            }
                            self.sort_processes();
                        }
                        ui.separator();
                        if ui
                            .selectable_label(
                                self.sort_column == SortColumn::Runtime,
                                format!(
                                    "Runtime {}", if self.sort_column == SortColumn::Runtime {
                                    if self.sort_ascending { "▲" } else { "▼" } } else { ""
                                    }
                                ),
                            )
                            .clicked()
                        {
                            if self.sort_column == SortColumn::Runtime {
                                self.sort_ascending = !self.sort_ascending;
                            } else {
                                self.sort_column = SortColumn::Runtime;
                                self.sort_ascending = false;
                            }
                            self.sort_processes();
                        }
                    });
                    ui.separator();
                    let filter_lower = self.search_filter.to_lowercase();
                    let mut process_to_kill: Option<u32> = None;
                    let mut program_to_add: Option<(String, String)> = None;
                    let filtered_processes: Vec<&ProcessInfo> = self
                        .processes
                        .iter()
                        .filter(|process| {
                            if filter_lower.is_empty() {
                                true
                            } else {
                                let name_match = process
                                    .name
                                    .to_lowercase()
                                    .contains(&filter_lower);
                                let pid_match = process
                                    .pid
                                    .to_string()
                                    .contains(&filter_lower);
                                name_match || pid_match
                            }
                        })
                        .collect();
                    let start = self.current_page * self.items_per_page;
                    let end = (start + self.items_per_page)
                        .min(filtered_processes.len());
                    let paginated = if start >= filtered_processes.len() {
                        &[]
                    } else {
                        &filtered_processes[start..end]
                    };
                    for process in paginated {
                        let is_selected = self.selected_pid == Some(process.pid);
                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing.x = 10.0;
                            if process.is_foreground {
                                ui.colored_label(
                                    egui::Color32::from_rgb(100, 200, 255),
                                    "🔷",
                                );
                            } else {
                                ui.label("  ");
                            }
                            let response = ui
                                .selectable_label(is_selected, format!("{}", process.pid));
                            if response.clicked() {
                                self.selected_pid = Some(process.pid);
                            }
                            ui.separator();
                            ui.label(&process.name);
                            ui.separator();
                            ui.label(Self::format_memory(process.memory));
                            ui.separator();
                            let cpu_color = if process.cpu_usage > 50.0 {
                                egui::Color32::RED
                            } else if process.cpu_usage > 20.0 {
                                egui::Color32::YELLOW
                            } else {
                                egui::Color32::GREEN
                            };
                            ui.colored_label(
                                cpu_color,
                                format!("{:.2}%", process.cpu_usage),
                            );
                            ui.separator();
                            if let Some(parent) = process.parent_pid {
                                ui.label(format!("{}", parent));
                            } else {
                                ui.label("-");
                            }
                            ui.separator();
                            ui.label(&process.status);
                            ui.separator();
                            ui.label(Self::format_time(process.run_time));
                            ui.separator();
                            if ui.button("❌ Kill").clicked() {
                                process_to_kill = Some(process.pid);
                            }
                            if let Some(path) = &process.exe_path {
                                if ui
                                    .button("⭐")
                                    .on_hover_text("Add to Custom Programs")
                                    .clicked()
                                {
                                    program_to_add = Some((process.name.clone(), path.clone()));
                                }
                            }
                        });
                        ui.separator();
                    }
                    if let Some(pid) = process_to_kill {
                        if let Some(window) = self.windows.iter().find(|w| w.pid == pid)
                        {
                            if let Some(image_data) = self
                                .capture_window_thumbnail(window.window_id, 300, 200)
                            {
                                let texture = ctx
                                    .load_texture(
                                        format!("kill_confirm_{}", pid),
                                        image_data,
                                        egui::TextureOptions::default(),
                                    );
                                self.kill_confirm_thumbnail = Some(texture);
                            }
                        }
                        self.kill_confirm_pid = Some(pid);
                    }
                    if let Some((name, path)) = program_to_add {
                        self.add_custom_program(name, path, String::new(), false);
                        self.view_mode = ViewMode::New;
                    }
                },
            );
    }

    #[cfg(windows)]
    pub fn capture_window_thumbnail(&self, window_id: u64, max_width: i32, max_height: i32) -> Option<egui::ColorImage> {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::Graphics::Gdi::{
            CreateCompatibleDC, CreateCompatibleBitmap, SelectObject, DeleteDC, DeleteObject,
            GetDC, ReleaseDC, BitBlt, SRCCOPY, GetDIBits, BITMAPINFO, BITMAPINFOHEADER,
            BI_RGB, DIB_RGB_COLORS,
        };
        use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;
        
        unsafe {
            let hwnd = HWND(window_id as *mut _);
            
            // Get window size
            let mut rect = std::mem::zeroed();
            if GetWindowRect(hwnd, &mut rect).is_err() {
                return None;
            }
            
            let width = (rect.right - rect.left) as i32;
            let height = (rect.bottom - rect.top) as i32;
            
            if width <= 0 || height <= 0 {
                return None;
            }
            
            // Scale down for thumbnail
            let max_width = max_width.max(32) as f32;
            let max_height = max_height.max(32) as f32;
            let scale = f32::min(max_width / width as f32, max_height / height as f32).min(1.0);
            let thumb_width = (width as f32 * scale) as i32;
            let thumb_height = (height as f32 * scale) as i32;
            
            if thumb_width <= 0 || thumb_height <= 0 {
                return None;
            }
            
            // Create DC and bitmap
            let hdc_window = GetDC(hwnd);
            if hdc_window.0.is_null() {
                return None;
            }
            
            let hdc_mem = CreateCompatibleDC(hdc_window);
            if hdc_mem.0.is_null() {
                ReleaseDC(hwnd, hdc_window);
                return None;
            }
            
            let hbitmap = CreateCompatibleBitmap(hdc_window, width, height);
            if hbitmap.0.is_null() {
                DeleteDC(hdc_mem);
                ReleaseDC(hwnd, hdc_window);
                return None;
            }
            
            let old_bitmap = SelectObject(hdc_mem, hbitmap);
            
            // Capture window content using BitBlt
            let _ = BitBlt(hdc_mem, 0, 0, width, height, hdc_window, 0, 0, SRCCOPY);
            
            // Get bitmap data
            let mut bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height, // Top-down
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [std::mem::zeroed()],
            };
            
            let mut buffer: Vec<u8> = vec![0; (width * height * 4) as usize];
            
            let result = GetDIBits(
                hdc_mem,
                hbitmap,
                0,
                height as u32,
                Some(buffer.as_mut_ptr() as *mut _),
                &mut bmi,
                DIB_RGB_COLORS,
            );
            
            // Cleanup
            SelectObject(hdc_mem, old_bitmap);
            DeleteObject(hbitmap);
            DeleteDC(hdc_mem);
            ReleaseDC(hwnd, hdc_window);
            
            if result == 0 {
                return None;
            }
            
            // Convert BGRA to RGBA and resize
            let mut pixels = Vec::with_capacity((thumb_width * thumb_height) as usize);
            
            for y in 0..thumb_height {
                for x in 0..thumb_width {
                    // Simple nearest-neighbor scaling
                    let src_x = (x as f32 / scale) as i32;
                    let src_y = (y as f32 / scale) as i32;
                    let src_idx = ((src_y * width + src_x) * 4) as usize;
                    
                    if src_idx + 3 < buffer.len() {
                        let b = buffer[src_idx];
                        let g = buffer[src_idx + 1];
                        let r = buffer[src_idx + 2];
                        pixels.push(egui::Color32::from_rgb(r, g, b));
                    } else {
                        pixels.push(egui::Color32::from_gray(50));
                    }
                }
            }
            
            Some(egui::ColorImage {
                size: [thumb_width as usize, thumb_height as usize],
                pixels,
            })
        }
    }
    #[cfg(not(windows))]
    pub fn capture_window_thumbnail(
        &self,
        _window_id: u64,
        _max_width: i32,
        _max_height: i32,
    ) -> Option<egui::ColorImage> {
        None
    }
    pub fn add_custom_program(
        &mut self,
        name: String,
        path: String,
        args: String,
        admin: bool,
    ) {
        if !self.custom_programs.iter().any(|p| p.path == path && p.args == args) {
            self.custom_programs
                .push(CustomProgram {
                    name,
                    path,
                    args,
                    admin,
                });
            self.save_config();
            self.add_log("Added new custom program".to_string());
        } else {
            self.add_log("Program already exists in custom list".to_string());
        }
    }
}
