#![allow(dead_code)]

/// Strongly typed color for Ableton Push 2 pads and buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PushColor(pub u8);

pub const BLACK: PushColor = PushColor(0);
pub const PINK: PushColor = PushColor(1);
pub const RED: PushColor = PushColor(2);
pub const ORANGE: PushColor = PushColor(3);

pub const ORANGE2: PushColor = PushColor(4);
pub const BROWN_PALE: PushColor = PushColor(5);
pub const BROWN: PushColor = PushColor(6);
pub const YELLOW_PALE: PushColor = PushColor(7);

pub const YELLOW: PushColor = PushColor(8);
pub const GREEN_LIME: PushColor = PushColor(9);
pub const GREEN_LIGHT: PushColor = PushColor(10);
pub const GREEN: PushColor = PushColor(11);

pub const GREEN_TURTLE: PushColor = PushColor(12);
pub const GREEN_PALE: PushColor = PushColor(13);
pub const TURQUOISE_PALE: PushColor = PushColor(14);
pub const TURQUOISE: PushColor = PushColor(15);

pub const BLUE_SKY: PushColor = PushColor(16);
pub const PURPLE_PALE: PushColor = PushColor(17);
pub const PURPLE_BLUE: PushColor = PushColor(18);
pub const PURPLE: PushColor = PushColor(19);

pub const BLUE_SKY_DARK: PushColor = PushColor(20);
pub const YELLOW_AMBER_BRIGHT: PushColor = PushColor(21);
pub const YELLOW_LOW: PushColor = PushColor(22);
pub const YELLOW2: PushColor = PushColor(23);

pub const YELLOW_BRIGHT: PushColor = PushColor(24);
pub const YELLOW_LIME_LOW: PushColor = PushColor(25);
pub const YELLOW_LIME: PushColor = PushColor(26);
pub const YELLOW_LIME_BRIGHT: PushColor = PushColor(27);

pub const LIME_YELLOW_LOW: PushColor = PushColor(28);
pub const LIME_YELLOW: PushColor = PushColor(29);
pub const LIME_YELLOW_BRIGHT: PushColor = PushColor(30);
pub const LIME_LOW: PushColor = PushColor(31);

pub const LIME: PushColor = PushColor(32);
pub const LIME_BRIGHT: PushColor = PushColor(33);
pub const LIME_GREEN_LOW: PushColor = PushColor(34);
pub const LIME_GREEN: PushColor = PushColor(35);

pub const LIME_GREEN_BRIGHT: PushColor = PushColor(36);
pub const GREEN_LIME_LOW: PushColor = PushColor(37);
pub const GREEN_LIME2: PushColor = PushColor(38);
pub const GREEN_LIME_BRIGHT: PushColor = PushColor(39);

pub const GREEN_LOW: PushColor = PushColor(40);
pub const GREEN2: PushColor = PushColor(41);
pub const GREEN_BRIGHT: PushColor = PushColor(42);
pub const GREEN_SPRING_LOW: PushColor = PushColor(43);

pub const GREEN_SPRING: PushColor = PushColor(44);
pub const GREEN_SPRING_BRIGHT: PushColor = PushColor(45);
pub const SPRING_GREEN_LOW: PushColor = PushColor(46);
pub const SPRING_GREEN: PushColor = PushColor(47);

pub const SPRING_GREEN_BRIGHT: PushColor = PushColor(48);
pub const SPRING_LOW: PushColor = PushColor(49);
pub const SPRING: PushColor = PushColor(50);
pub const SPRING_BRIGHT: PushColor = PushColor(51);

pub const SPRING_CYAN_LOW: PushColor = PushColor(52);
pub const SPRING_CYAN: PushColor = PushColor(53);
pub const SPRING_CYAN_BRIGHT: PushColor = PushColor(54);
pub const CYAN_SPRING_LOW: PushColor = PushColor(55);

pub const CYAN_SPRING: PushColor = PushColor(56);
pub const CYAN_SPRING_BRIGHT: PushColor = PushColor(57);
pub const CYAN_LOW: PushColor = PushColor(58);
pub const CYAN: PushColor = PushColor(59);

pub const CYAN_BRIGHT: PushColor = PushColor(60);
pub const CYAN_AZURE_LOW: PushColor = PushColor(61);
pub const CYAN_AZURE: PushColor = PushColor(62);
pub const CYAN_AZURE_BRIGHT: PushColor = PushColor(63);

pub const AZURE_CYAN_LOW: PushColor = PushColor(64);
pub const AZURE_CYAN: PushColor = PushColor(65);
pub const AZURE_CYAN_BRIGHT: PushColor = PushColor(66);
pub const AZURE_LOW: PushColor = PushColor(67);

pub const AZURE: PushColor = PushColor(68);
pub const AZURE_BRIGHT: PushColor = PushColor(69);
pub const AZURE_BLUE_LOW: PushColor = PushColor(70);
pub const AZURE_BLUE: PushColor = PushColor(71);

pub const AZURE_BLUE_BRIGHT: PushColor = PushColor(72);
pub const BLUE_AZURE_LOW: PushColor = PushColor(73);
pub const BLUE_AZURE: PushColor = PushColor(74);
pub const BLUE_AZURE_BRIGHT: PushColor = PushColor(75);

pub const BLUE_LOW: PushColor = PushColor(76);
pub const BLUE: PushColor = PushColor(77);
pub const BLUE_BRIGHT: PushColor = PushColor(78);
pub const BLUE_VIOLET_LOW: PushColor = PushColor(79);

pub const BLUE_VIOLET: PushColor = PushColor(80);
pub const BLUE_VIOLET_BRIGHT: PushColor = PushColor(81);
pub const VIOLET_BLUE_LOW: PushColor = PushColor(82);
pub const VIOLET_BLUE: PushColor = PushColor(83);

pub const VIOLET_BLUE_BRIGHT: PushColor = PushColor(84);
pub const VIOLET_LOW: PushColor = PushColor(85);
pub const VIOLET: PushColor = PushColor(86);
pub const VIOLET_BRIGHT: PushColor = PushColor(87);

pub const VIOLET_MAGENTA_LOW: PushColor = PushColor(88);
pub const VIOLET_MAGENTA: PushColor = PushColor(89);
pub const VIOLET_MAGENTA_BRIGHT: PushColor = PushColor(90);
pub const MAGENTA_VIOLET_LOW: PushColor = PushColor(91);

pub const MAGENTA_VIOLET: PushColor = PushColor(92);
pub const MAGENTA_VIOLET_BRIGHT: PushColor = PushColor(93);
pub const MAGENTA_LOW: PushColor = PushColor(94);
pub const MAGENTA: PushColor = PushColor(95);

pub const MAGENTA_BRIGHT: PushColor = PushColor(96);
pub const MAGENTA_PINK_LOW: PushColor = PushColor(97);
pub const MAGENTA_PINK: PushColor = PushColor(98);
pub const MAGENTA_PINK_BRIGHT: PushColor = PushColor(99);

pub const PINK_MAGENTA_LOW: PushColor = PushColor(100);
pub const PINK_MAGENTA: PushColor = PushColor(101);
pub const PINK_MAGENTA_BRIGHT: PushColor = PushColor(102);
pub const PINK_LOW: PushColor = PushColor(103);

pub const PINK2: PushColor = PushColor(104);
pub const PINK_BRIGHT: PushColor = PushColor(105);
pub const PINK_RED_LOW: PushColor = PushColor(106);
pub const PINK_RED: PushColor = PushColor(107);

pub const PINK_RED_BRIGHT: PushColor = PushColor(108);
pub const RED_PINK_LOW: PushColor = PushColor(109);
pub const RED_PINK: PushColor = PushColor(110);
pub const RED_PINK_BRIGHT: PushColor = PushColor(111);

pub const RED_LOW: PushColor = PushColor(112);
pub const RED2: PushColor = PushColor(113);
pub const RED_BRIGHT: PushColor = PushColor(114);
pub const WARM_WHITE_LOW: PushColor = PushColor(115);

pub const WARM_WHITE: PushColor = PushColor(116);
pub const WARM_WHITE_BRIGHT: PushColor = PushColor(117);
pub const WHITE_LOW: PushColor = PushColor(118);
pub const WHITE_BRIGHT: PushColor = PushColor(119);

pub const ORANGE_LOW: PushColor = PushColor(120);
pub const ORANGE3: PushColor = PushColor(121);
pub const ORANGE_BRIGHT: PushColor = PushColor(122);
pub const YELLOW_PALE2: PushColor = PushColor(123);
pub const LIME_PALE: PushColor = PushColor(124);
pub const GREEN_PALE2: PushColor = PushColor(125);
pub const CYAN_PALE: PushColor = PushColor(126);
pub const BLUE_PALE: PushColor = PushColor(127);
