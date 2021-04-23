use std::cmp;

pub struct ViewportModel {
    pub width: i32,
    pub height: i32,
    pub y_offset: i32,
    pub y_position: i32,
    pub high_performance_rendering: bool,
    lines: Vec<String>,
}

impl ViewportModel {
    pub fn at_top(&self) -> bool {
        self.y_offset <= 0
    }

    pub fn at_bottom(&self) -> bool {
        self.y_offset >= self.lines.len() - 1 - &self.height
    }

    pub fn past_bottom(&self) -> bool {
        self.y_offset > self.lines.len() - 1 - &self.height
    }

    pub fn scroll_percent(&self) -> f64 {
        if self.height >= self.lines.len() {
            1.0
        }
        let y = &self.y_offset as f64;
        let h = &self.height as f64;
        let t = (self.lines.len() - 1) as f64;
        let v = y / (t - h);
        cmp::max(0.0, cmp::min(1.0, v))
    }

    pub fn set_content(&self, s: &str) {
        let s = str::replace(s, "\r\n", "\n");
        self.lines = s.split("\n");

        if self.y_offset > self.lines.len() - 1 {
            self.go_to_bottom();
        }
    }

    fn visible_lines(&self) -> Vec<String> {
        if self.lines.len() > 0 {
            let top = cmp::max(0, &self.y_offset);
            let bottom = cmp::min(self.lines.len(), cmp::max(top, &self.y_offset + &self.height));
            self.lines[top..bottom]
        } else {
            Vec::new()
        }
    }

    pub fn view_down(&mut self) -> Option<Vec<String>> {
        if self.at_bottom() {
            None()
        } else {
            self.y_offset = cmp::min(&self.y_offset + &self.height, self.lines.len() - 1 - &self.height);
            Some(self.visible_lines())
        }
    }

    pub fn view_up(&mut self) -> Option<Vec<String>> {
        if self.at_top() {
            None()
        } else {
            self.y_offset = cmp::max(&self.y_offset - &self.height, 0);
            Some(self.visible_lines())
        }
    }

    pub fn half_view_down(&mut self) -> Option<Vec<String>> {
        if self.at_bottom() {
            None()
        } else {
            self.y_offset = cmp::min(&self.y_offset + &self.height / 2, self.lines.len() - 1 - &self.height);
            if self.lines.len() > 0 {
                let top = cmp::max(&self.y_offset + &self.height / 2, 0);
                let bottom = cmp::min(self.lines.len() - 1, cmp::max(top, &self.y_offset + &self.height));
                Some(self.lines[top..bottom])
            } else {
                Some(Vec::new());
            }
        }
    }

    pub fn half_view_up(&mut self) -> Option<Vec<String>> {
        if self.at_top() {
            None()
        } else {
            self.y_offset = cmp::max(&self.y_offset - &self.height / 2, 0);
            if self.lines.len() > 0 {
                let top = cmp::max(&self.y_offset, 0);
                let bottom = cmp::min(self.lines.len() - 1, cmp::max(top, &self.y_offset + &self.height / 2));
                Some(self.lines[top..bottom])
            } else {
                Some(Vec::new())
            }
        }
    }

    pub fn line_down(&mut self, n: u32) -> Option<Vec<String>> {
        if self.at_bottom() || n == 0 {
            None()
        } else {
            let max_delta = (self.lines.len() - 1) - (&self.y_offset + &self.height);
            let n = cmp::min(n, max_delta);

            self.y_offset = cmp::min(&self.y_offset + n, self.lines.len() - 1 - &self.height);

            if self.lines.len() > 0 {
                let top = cmp::max(&self.y_offset + &self.height - n, 0);
                let bottom = cmp::min(self.lines.len() - 1, cmp::max(top, &self.y_offset + &self.height));
                Some(self.lines[top..bottom])
            } else {
                Some(Vec::new())
            }
        }
    }

    pub fn line_up(&mut self, n: u32) -> Option<Vec<String>> {
        if self.at_top() || n == 0 {
            None()
        } else {
            let n = cmp::min(n, &self.y_offset);

            self.y_offset = cmp::max(&self.y_offset - n, 0);

            if self.lines.len() > 0 {
                let top = cmp::max(0, &self.y_offset);
                let bottom = cmp::min(self.lines.len() - 1, cmp::min(top, &self.y_offset + n));
                Some(self.lines[top..bottom])
            } else {
                Some(Vec::new())
            }
        }
    }

    pub fn go_to_top(&mut self) -> Option<Vec<String>> {
        if self.at_top() {
            None()
        } else {
            self.y_offset = 0;

            if self.lines.len() > 0 {
                let top = &self.y_offset;
                let bottom = cmp::min(self.lines.len() - 1, cmp::max(top, &self.y_offset + &self.height));
                Some(self.lines[top..bottom])
            } else {
                Some(Vec::new())
            }
        }
    }

    pub fn go_to_bottom(&mut self) -> Vec<String> {
        self.y_offset = cmp::max(self.lines.len() - 1 - &self.height, 0);

        if self.lines.len() > 0 {
            let top = &self.y_offset;
            let bottom = cmp::max(self.lines.len() - 1, 0);
            self.lines[top..bottom]
        } else {
            Vec::new()
        }
    }
}

impl Model for ViewportModel {
    fn view(&self) -> String {
        if self.high_performance_rendering {
            std::iter::repeat("\n").take(&self.height - 1).collect::<String>()
        } else {
            let lines = self.visible_lines();
            let extra_lines = "";
            if lines.len() < &self.height {
                let extra_lines = std::iter::repeat("\n").take(&self.height - lines.len());
            }

            lines.join("\n") + extra_lines
        }
    }

    fn update(&self, message: Message) -> String {

    }
}