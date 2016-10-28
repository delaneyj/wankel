use math::*;

#[derive(Debug,PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    // maroon  #800000 (128,0,0)
    // dark red #8B0000 (139,0,0)
    // brown #A52A2A (165,42,42)
    // firebrick #B22222 (178,34,34)
    // crimson #DC143C (220,20,60)
    // red #FF0000 (255,0,0)
    // tomato #FF6347 (255,99,71)
    // coral #FF7F50 (255,127,80)
    // indian red #CD5C5C (205,92,92)
    // light coral #F08080 (240,128,128)
    // dark salmon #E9967A (233,150,122)
    // salmon #FA8072 (250,128,114)
    // light salmon #FFA07A (255,160,122)
    // orange red #FF4500 (255,69,0)
    // dark orange #FF8C00 (255,140,0)
    // orange #FFA500 (255,165,0)
    // gold #FFD700 (255,215,0)
    // dark golden rod #B8860B (184,134,11)
    // golden rod #DAA520 (218,165,32)
    // pale golden rod #EEE8AA (238,232,170)
    // dark khaki #BDB76B (189,183,107)
    // khaki #F0E68C (240,230,140)
    // olive #808000 (128,128,0)
    // yellow #FFFF00 (255,255,0)
    // yellow green #9ACD32 (154,205,50)
    // dark olive green #556B2F (85,107,47)
    // olive drab #6B8E23 (107,142,35)
    // lawn green #7CFC00 (124,252,0)
    // chart reuse #7FFF00 (127,255,0)
    // green yellow #ADFF2F (173,255,47)
    // dark green #006400 (0,100,0)
    // green #008000 (0,128,0)
    // forest green #228B22 (34,139,34)
    // lime #00FF00 (0,255,0)
    // lime green #32CD32 (50,205,50)
    // light green #90EE90 (144,238,144)
    // pale green #98FB98 (152,251,152)
    // dark sea green #8FBC8F (143,188,143)
    // medium spring green #00FA9A (0,250,154)
    // spring green #00FF7F (0,255,127)
    // sea green #2E8B57 (46,139,87)
    // medium aqua marine #66CDAA (102,205,170)
    // medium sea green #3CB371 (60,179,113)
    // light sea green #20B2AA (32,178,170)
    // dark slate gray #2F4F4F (47,79,79)
    // teal #008080 (0,128,128)
    // dark cyan #008B8B (0,139,139)
    // aqua #00FFFF (0,255,255)
    // cyan #00FFFF (0,255,255)
    // light cyan #E0FFFF (224,255,255)
    // dark turquoise #00CED1 (0,206,209)
    // turquoise #40E0D0 (64,224,208)
    // medium turquoise #48D1CC (72,209,204)
    // pale turquoise #AFEEEE (175,238,238)
    // aqua marine #7FFFD4 (127,255,212)
    // powder blue #B0E0E6 (176,224,230)
    // cadet blue #5F9EA0 (95,158,160)
    // steel blue #4682B4 (70,130,180)
    // corn flower blue #6495ED (100,149,237)
    // deep sky blue #00BFFF (0,191,255)
    // dodger blue #1E90FF  (30,144,255)
    // light blue #ADD8E6 (173,216,230)
    // sky blue #87CEEB (135,206,235)
    // light sky blue #87CEFA (135,206,250)
    // midnight blue #191970 (25,25,112)
    // navy #000080 (0,0,128)
    // dark blue #00008B (0,0,139)
    // medium blue #0000CD (0,0,205)
    // blue #0000FF (0,0,255)
    // royal blue #4169E1 (65,105,225)
    // blue violet #8A2BE2 (138,43,226)
    // indigo #4B0082 (75,0,130)
    // dark slate blue #483D8B (72,61,139)
    // slate blue #6A5ACD (106,90,205)
    // medium slate blue #7B68EE (123,104,238)
    // medium purple #9370DB (147,112,219)
    // dark magenta #8B008B (139,0,139)
    // dark violet #9400D3 (148,0,211)
    // dark orchid #9932CC (153,50,204)
    // medium orchid #BA55D3 (186,85,211)
    // purple #800080 (128,0,128)
    // thistle #D8BFD8 (216,191,216)
    // plum #DDA0DD (221,160,221)
    // violet #EE82EE (238,130,238)
    // magenta / fuchsia #FF00FF (255,0,255)
    // orchid #DA70D6 (218,112,214)
    // medium violet red #C71585 (199,21,133)
    // pale violet red #DB7093 (219,112,147)
    // deep pink #FF1493  (255,20,147)
    // hot pink #FF69B4 (255,105,180)
    // light pink  #FFB6C1 (255,182,193)
    // pink #FFC0CB (255,192,203)
    // antique white #FAEBD7 (250,235,215)
    // beige #F5F5DC (245,245,220)
    // bisque #FFE4C4  (255,228,196)
    // blanched almond #FFEBCD (255,235,205)
    // wheat #F5DEB3 (245,222,179)
    // corn silk #FFF8DC (255,248,220)
    // lemon chiffon #FFFACD (255,250,205)
    // light golden rod yellow #FAFAD2 (250,250,210)
    // light yellow #FFFFE0 (255,255,224)
    // saddle brown #8B4513 (139,69,19)
    // sienna #A0522D (160,82,45)
    // chocolate #D2691E (210,105,30)
    // peru #CD853F (205,133,63)
    // sandy brown #F4A460 (244,164,96)
    // burly wood #DEB887 (222,184,135)
    // tan #D2B48C (210,180,140)
    // rosy brown #BC8F8F (188,143,143)
    // moccasin #FFE4B5 (255,228,181)
    // navajo white #FFDEAD (255,222,173)
    // peach puff #FFDAB9 (255,218,185)
    // misty rose #FFE4E1 (255,228,225)
    // lavender blush #FFF0F5 (255,240,245)
    // linen #FAF0E6 (250,240,230)
    // old lace #FDF5E6 (253,245,230)
    // papaya whip #FFEFD5 (255,239,213)
    // sea shell #FFF5EE (255,245,238)
    // mint cream #F5FFFA (245,255,250)
    // slate gray #708090 (112,128,144)
    // light slate gray #778899 (119,136,153)
    // light steel blue #B0C4DE (176,196,222)
    // lavender #E6E6FA (230,230,250)
    // floral white #FFFAF0 (255,250,240)
    // alice blue #F0F8FF (240,248,255)
    // ghost white #F8F8FF (248,248,255)
    // honeydew #F0FFF0 (240,255,240)
    // ivory #FFFFF0 (255,255,240)
    // azure #F0FFFF (240,255,255)
    // snow #FFFAFA (255,250,250)
    // black #000000 (0,0,0)
    // dim gray / dim grey #696969 (105,105,105)
    // gray / grey #808080 (128,128,128)
    // dark gray / dark grey #A9A9A9 (169,169,169)
    // silver #C0C0C0 (192,192,192)
    // light gray / light grey #D3D3D3 (211,211,211)
    // gainsboro #DCDCDC (220,220,220)
    // white smoke #F5F5F5 (245,245,245)
    // white #FFFFFF (255,255,255)
    //
    pub fn from_floats(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: r.max(0.0).min(1.0),
            g: g.max(0.0).min(1.0),
            b: b.max(0.0).min(1.0),
        }
    }

    pub fn from_ints(r: u32, g: u32, b: u32) -> Color {
        Color::from_floats((r as f32) / 255.0, (g as f32) / 255.0, (b as f32) / 255.0)
    }

    pub fn from_hex(hex: u32) -> Color {
        Color::from_ints(hex >> 16 & 255, hex >> 8 & 255, hex & 255)
    }

    pub fn from_scalar(scalar: f32) -> Color {
        Color::from_floats(scalar, scalar, scalar)
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Color {
        let hue2rgb = |p: f32, q: f32, t: f32| -> f32 {
            let mut mt = t;
            if mt < 0.0 {
                mt += 1.0;
            }
            if mt > 1.0 {
                mt -= 1.0;
            }
            if mt < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * mt;
            }
            if mt < 1.0 / 2.0 {
                return q;
            }
            if mt < 2.0 / 3.0 {
                return p + (q - p) * 6.0 * (2.0 / 3.0 - mt);
            }
            p
        };



        // h,s,l ranges are in 0.0 - 1.0
        let h_clamped = euclidean_modulo(h, 1.0);
        let s_clamped = clamp(s, 0.0, 1.0);
        let l = clamp(l, 0.0, 1.0);

        if s_clamped == 0.0 {
            Color::from_scalar(1.0)
        } else {
            let p = if l <= 0.5 {
                l * (1.0 + s_clamped)
            } else {
                l + s_clamped - (l * s_clamped)
            };
            let q = (2.0 * l) - p;
            let one_third = 1.0 / 3.0;
            Color {
                r: hue2rgb(q, p, h_clamped + one_third),
                g: hue2rgb(q, p, h_clamped),
                b: hue2rgb(q, p, h_clamped - one_third),
            }
        }
    }


    // 	copyGammaToLinear: function ( color, gammaFactor ) {

    // 		if ( gammaFactor === undefined ) gammaFactor = 2.0;

    // 		this.r = Math.pow( color.r, gammaFactor );
    // 		this.g = Math.pow( color.g, gammaFactor );
    // 		this.b = Math.pow( color.b, gammaFactor );

    // 		return this;

    // 	},

    // 	copyLinearToGamma: function ( color, gammaFactor ) {

    // 		if ( gammaFactor === undefined ) gammaFactor = 2.0;

    // 		var safeInverse = ( gammaFactor > 0 ) ? ( 1.0 / gammaFactor ) : 1.0;

    // 		this.r = Math.pow( color.r, safeInverse );
    // 		this.g = Math.pow( color.g, safeInverse );
    // 		this.b = Math.pow( color.b, safeInverse );

    // 		return this;

    // 	},

    // 	convertGammaToLinear: function () {

    // 		var r = this.r, g = this.g, b = this.b;

    // 		this.r = r * r;
    // 		this.g = g * g;
    // 		this.b = b * b;

    // 		return this;

    // 	},

    // 	convertLinearToGamma: function () {

    // 		this.r = Math.sqrt( this.r );
    // 		this.g = Math.sqrt( this.g );
    // 		this.b = Math.sqrt( this.b );

    // 		return this;

    // 	},

    // 	getHex: function () {

    // 		return ( this.r * 255 ) << 16 ^ ( this.g * 255 ) << 8 ^ ( this.b * 255 ) << 0;

    // 	},

    // 	getHexString: function () {

    // 		return ( '000000' + this.getHex().toString( 16 ) ).slice( - 6 );

    // 	},

    // 	getHSL: function ( optionalTarget ) {

    // 		// h,s,l ranges are in 0.0 - 1.0

    // 		var hsl = optionalTarget || { h: 0, s: 0, l: 0 };

    // 		var r = this.r, g = this.g, b = this.b;

    // 		var max = Math.max( r, g, b );
    // 		var min = Math.min( r, g, b );

    // 		var hue, saturation;
    // 		var lightness = ( min + max ) / 2.0;

    // 		if ( min === max ) {

    // 			hue = 0;
    // 			saturation = 0;

    // 		} else {

    // 			var delta = max - min;

    // 			saturation = lightness <= 0.5 ? delta / ( max + min ) : delta / ( 2 - max - min );

    // 			switch ( max ) {

    // 				case r: hue = ( g - b ) / delta + ( g < b ? 6 : 0 ); break;
    // 				case g: hue = ( b - r ) / delta + 2; break;
    // 				case b: hue = ( r - g ) / delta + 4; break;

    // 			}

    // 			hue /= 6;

    // 		}

    // 		hsl.h = hue;
    // 		hsl.s = saturation;
    // 		hsl.l = lightness;

    // 		return hsl;

    // 	},

    // 	offsetHSL: function ( h, s, l ) {

    // 		var hsl = this.getHSL();

    // 		hsl.h += h; hsl.s += s; hsl.l += l;

    // 		this.setHSL( hsl.h, hsl.s, hsl.l );

    // 		return this;

    // 	},

    // 	add: function ( color ) {

    // 		this.r += color.r;
    // 		this.g += color.g;
    // 		this.b += color.b;

    // 		return this;

    // 	},

    // 	addColors: function ( color1, color2 ) {

    // 		this.r = color1.r + color2.r;
    // 		this.g = color1.g + color2.g;
    // 		this.b = color1.b + color2.b;

    // 		return this;

    // 	},

    // 	addScalar: function ( s ) {

    // 		this.r += s;
    // 		this.g += s;
    // 		this.b += s;

    // 		return this;

    // 	},

    // 	sub: function( color ) {

    // 		this.r = Math.max( 0, this.r - color.r );
    // 		this.g = Math.max( 0, this.g - color.g );
    // 		this.b = Math.max( 0, this.b - color.b );

    // 		return this;

    // 	},

    // 	multiply: function ( color ) {

    // 		this.r *= color.r;
    // 		this.g *= color.g;
    // 		this.b *= color.b;

    // 		return this;

    // 	},

    // 	multiplyScalar: function ( s ) {

    // 		this.r *= s;
    // 		this.g *= s;
    // 		this.b *= s;

    // 		return this;

    // 	},

    // 	lerp: function ( color, alpha ) {

    // 		this.r += ( color.r - this.r ) * alpha;
    // 		this.g += ( color.g - this.g ) * alpha;
    // 		this.b += ( color.b - this.b ) * alpha;

    // 		return this;

    // 	},

    // 	equals: function ( c ) {

    // 		return ( c.r === this.r ) && ( c.g === this.g ) && ( c.b === this.b );

    // 	},

    // 	fromArray: function ( array, offset ) {

    // 		if ( offset === undefined ) offset = 0;

    // 		this.r = array[ offset ];
    // 		this.g = array[ offset + 1 ];
    // 		this.b = array[ offset + 2 ];

    // 		return this;

    // 	},

    // 	toArray: function ( array, offset ) {

    // 		if ( array === undefined ) array = [];
    // 		if ( offset === undefined ) offset = 0;

    // 		array[ offset ] = this.r;
    // 		array[ offset + 1 ] = this.g;
    // 		array[ offset + 2 ] = this.b;

    // 		return array;

    // 	},

    // 	toJSON: function () {

    // 		return this.getHex();

    // 	}

    // };

    // var ColorKeywords = { 'aliceblue': 0xF0F8FF, 'antiquewhite': 0xFAEBD7, 'aqua': 0x00FFFF, 'aquamarine': 0x7FFFD4, 'azure': 0xF0FFFF,
    // 'beige': 0xF5F5DC, 'bisque': 0xFFE4C4, 'black': 0x000000, 'blanchedalmond': 0xFFEBCD, 'blue': 0x0000FF, 'blueviolet': 0x8A2BE2,
    // 'brown': 0xA52A2A, 'burlywood': 0xDEB887, 'cadetblue': 0x5F9EA0, 'chartreuse': 0x7FFF00, 'chocolate': 0xD2691E, 'coral': 0xFF7F50,
    // 'cornflowerblue': 0x6495ED, 'cornsilk': 0xFFF8DC, 'crimson': 0xDC143C, 'cyan': 0x00FFFF, 'darkblue': 0x00008B, 'darkcyan': 0x008B8B,
    // 'darkgoldenrod': 0xB8860B, 'darkgray': 0xA9A9A9, 'darkgreen': 0x006400, 'darkgrey': 0xA9A9A9, 'darkkhaki': 0xBDB76B, 'darkmagenta': 0x8B008B,
    // 'darkolivegreen': 0x556B2F, 'darkorange': 0xFF8C00, 'darkorchid': 0x9932CC, 'darkred': 0x8B0000, 'darksalmon': 0xE9967A, 'darkseagreen': 0x8FBC8F,
    // 'darkslateblue': 0x483D8B, 'darkslategray': 0x2F4F4F, 'darkslategrey': 0x2F4F4F, 'darkturquoise': 0x00CED1, 'darkviolet': 0x9400D3,
    // 'deeppink': 0xFF1493, 'deepskyblue': 0x00BFFF, 'dimgray': 0x696969, 'dimgrey': 0x696969, 'dodgerblue': 0x1E90FF, 'firebrick': 0xB22222,
    // 'floralwhite': 0xFFFAF0, 'forestgreen': 0x228B22, 'fuchsia': 0xFF00FF, 'gainsboro': 0xDCDCDC, 'ghostwhite': 0xF8F8FF, 'gold': 0xFFD700,
    // 'goldenrod': 0xDAA520, 'gray': 0x808080, 'green': 0x008000, 'greenyellow': 0xADFF2F, 'grey': 0x808080, 'honeydew': 0xF0FFF0, 'hotpink': 0xFF69B4,
    // 'indianred': 0xCD5C5C, 'indigo': 0x4B0082, 'ivory': 0xFFFFF0, 'khaki': 0xF0E68C, 'lavender': 0xE6E6FA, 'lavenderblush': 0xFFF0F5, 'lawngreen': 0x7CFC00,
    // 'lemonchiffon': 0xFFFACD, 'lightblue': 0xADD8E6, 'lightcoral': 0xF08080, 'lightcyan': 0xE0FFFF, 'lightgoldenrodyellow': 0xFAFAD2, 'lightgray': 0xD3D3D3,
    // 'lightgreen': 0x90EE90, 'lightgrey': 0xD3D3D3, 'lightpink': 0xFFB6C1, 'lightsalmon': 0xFFA07A, 'lightseagreen': 0x20B2AA, 'lightskyblue': 0x87CEFA,
    // 'lightslategray': 0x778899, 'lightslategrey': 0x778899, 'lightsteelblue': 0xB0C4DE, 'lightyellow': 0xFFFFE0, 'lime': 0x00FF00, 'limegreen': 0x32CD32,
    // 'linen': 0xFAF0E6, 'magenta': 0xFF00FF, 'maroon': 0x800000, 'mediumaquamarine': 0x66CDAA, 'mediumblue': 0x0000CD, 'mediumorchid': 0xBA55D3,
    // 'mediumpurple': 0x9370DB, 'mediumseagreen': 0x3CB371, 'mediumslateblue': 0x7B68EE, 'mediumspringgreen': 0x00FA9A, 'mediumturquoise': 0x48D1CC,
    // 'mediumvioletred': 0xC71585, 'midnightblue': 0x191970, 'mintcream': 0xF5FFFA, 'mistyrose': 0xFFE4E1, 'moccasin': 0xFFE4B5, 'navajowhite': 0xFFDEAD,
    // 'navy': 0x000080, 'oldlace': 0xFDF5E6, 'olive': 0x808000, 'olivedrab': 0x6B8E23, 'orange': 0xFFA500, 'orangered': 0xFF4500, 'orchid': 0xDA70D6,
    // 'palegoldenrod': 0xEEE8AA, 'palegreen': 0x98FB98, 'paleturquoise': 0xAFEEEE, 'palevioletred': 0xDB7093, 'papayawhip': 0xFFEFD5, 'peachpuff': 0xFFDAB9,
    // 'peru': 0xCD853F, 'pink': 0xFFC0CB, 'plum': 0xDDA0DD, 'powderblue': 0xB0E0E6, 'purple': 0x800080, 'red': 0xFF0000, 'rosybrown': 0xBC8F8F,
    // 'royalblue': 0x4169E1, 'saddlebrown': 0x8B4513, 'salmon': 0xFA8072, 'sandybrown': 0xF4A460, 'seagreen': 0x2E8B57, 'seashell': 0xFFF5EE,
    // 'sienna': 0xA0522D, 'silver': 0xC0C0C0, 'skyblue': 0x87CEEB, 'slateblue': 0x6A5ACD, 'slategray': 0x708090, 'slategrey': 0x708090, 'snow': 0xFFFAFA,
    // 'springgreen': 0x00FF7F, 'steelblue': 0x4682B4, 'tan': 0xD2B48C, 'teal': 0x008080, 'thistle': 0xD8BFD8, 'tomato': 0xFF6347, 'turquoise': 0x40E0D0,
    // 'violet': 0xEE82EE, 'wheat': 0xF5DEB3, 'white': 0xFFFFFF, 'whitesmoke': 0xF5F5F5, 'yellow': 0xFFFF00, 'yellowgreen': 0x9ACD32 };


    // export { ColorKeywords, Color };
}