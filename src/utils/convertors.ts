import { hsv, hwb } from 'color-convert';

export function hsvToRgbString(hsvString: string): string {

    // Parse HSV values from input string
    const hsvRegex = /hsv\((\d+),\s*(\d+),\s*(\d+)\)/;
    const match = hsvString.match(hsvRegex);
    if (!match) {
        return "";
    }

    const hue: number = parseInt(match[1]);
    const saturation: number = parseInt(match[2]);
    const value: number = parseInt(match[3]);

    const [red, green, blue] = hsv.rgb([hue, saturation, value]);

    const rgbString: string = `rgb(${red}, ${green}, ${blue})`;
    return rgbString;
}

export function hwbToRgbString(hwbString: string): string {

    // Parse HWB values from input string
    const hwbRegex = /hwb\((\d+),\s*(\d+\.?\d*?)%,\s*(\d+\.?\d*?)%\)/;
    const match = hwbString.match(hwbRegex);
    if (!match) {
        return "";
    }

    const hue: number = parseFloat(match[1]);
    const whiteness: number = parseFloat(match[2]);
    const blackness: number = parseFloat(match[3]);

    const [red, green, blue] = hwb.rgb([hue, whiteness, blackness]);

    const rgbString: string = `rgb(${red}, ${green}, ${blue})`;
    return rgbString;
}
