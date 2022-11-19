// aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
// ignore the fact that this mess exists
/*
use iced::{application, widget::{pick_list, scrollable, container, Container, text_input}, color, Color, overlay::menu, Background};

#[derive(Debug, Clone, Copy)]
pub enum QuiltTheme {
    Light,
    Dark
}

impl application::StyleSheet for QuiltTheme {
    type Style = ();

    fn appearance(&self, style: Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0xFFFFFF),
            text_color: color!(0x000000)
        }
    }
}

impl Default for QuiltTheme {
    fn default() -> Self {
        Self::Light
    }
}

/*
impl text_input::StyleSheet for QuiltTheme {
    type Style = ();
    
    fn active(&self, style: Self::Style) -> text_input::Appearance {
        
    }
    
    fn focused(&self, style: Self::Style) -> text_input::Appearance {
        
    }
    
    fn hovered(&self, style: Self::Style) -> text_input::Appearance {
        
    }
}
*/

impl pick_list::StyleSheet for QuiltTheme {
    type Style = ();

    fn active(&self, _style: ()) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: color!(0x000000),
            placeholder_color: color!(0x000000),
            background: Background::Color(color!(0xFFFFFF)),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: color!(0x9722FF),
            icon_size: 0.7,
        }
    }

    fn hovered(&self, _style: ()) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: color!(0x000000),
            placeholder_color: color!(0x000000),
            background: Background::Color(color!(0xFFFFFF)),
            border_radius: 2.0,
            border_width: 2.0,
            border_color: color!(0x9722FF),
            icon_size: 0.7,
        }
    }
}

impl scrollable::StyleSheet for QuiltTheme {
    type Style = ();

    fn active(&self, _style: Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(color!(0xFFFFFF))),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: color!(0x000000),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, _style: Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(color!(0xFFFFFF))),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: color!(0x000000),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}

impl container::StyleSheet for QuiltTheme {
    type Style = ();

    fn appearance(&self, style: Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: None,
            background: Some(Background::Color(color!(0xFFFFFF))),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

impl menu::StyleSheet for QuiltTheme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> menu::Appearance {
        menu::Appearance {
            text_color: color!(0x000000),
            background: Background::Color(color!(0xFFFFFF)),
            border_width: 1.0,
            border_radius: 0.0,
            border_color: color!(0x000000),
            selected_text_color: color!(0x000000),
            selected_background: Background::Color(color!(0xFFFFFF)),
        }
    }
}
*/