use dioxus::prelude::*;

// Each icon is a 24×24 viewBox SVG, stroke-based, no fill (inherits currentColor)
macro_rules! icon {
    ($name:ident, $path:expr) => {
        #[component]
        pub fn $name() -> Element {
            rsx! {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "18", height: "18",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    dangerous_inner_html: $path
                }
            }
        }
    };
}

icon!(IconSettings,  "<circle cx='12' cy='12' r='3'/><path d='M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z'/>"); 
icon!(IconClose,     "<line x1='18' y1='6' x2='6' y2='18'/><line x1='6' y1='6' x2='18' y2='18'/>");
icon!(IconChevronUp, "<polyline points='18 15 12 9 6 15'/>");
icon!(IconChevronDown, "<polyline points='6 9 12 15 18 9'/>");
icon!(IconBell,        "<path d='M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9'/><path d='M13.73 21a2 2 0 0 1-3.46 0'/>");
icon!(IconBellOff,     "<path d='M13.73 21a2 2 0 0 1-3.46 0'/><path d='M18.63 13A17.89 17.89 0 0 1 18 8'/><path d='M6.26 6.26A5.86 5.86 0 0 0 6 8c0 7-3 9-3 9h14'/><path d='M18 8a6 6 0 0 0-9.33-5'/><line x1='1' y1='1' x2='23' y2='23'/>");
