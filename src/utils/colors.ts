export function rgbCompatible(color?: string | null) {
  if (color?.includes("hsv")) return hsvToRgbString(color);
  if (color?.includes("hwb")) return hwbToRgbString(color);
  return color;
}

export function hsvToRgbString(hsvString: string): string {
  // Parse HSV values from input string
  const hsvRegex = /hsv\((\d+),\s*(\d+)%?,\s*(\d+)%?\)/;
  const match = hsvString.match(hsvRegex);
  if (!match) {
    return "";
  }

  const hue: number = parseInt(match[1]);
  const saturation: number = parseInt(match[2]) / 100;
  const value: number = parseInt(match[3]) / 100;

  const chroma: number = value * saturation;
  const hueSegment: number = hue / 60;
  const x: number = chroma * (1 - Math.abs((hueSegment % 2) - 1));
  const m: number = value - chroma;

  let red: number = 0;
  let green: number = 0;
  let blue: number = 0;

  if (hueSegment >= 0 && hueSegment < 1) {
    red = chroma;
    green = x;
  } else if (hueSegment >= 1 && hueSegment < 2) {
    red = x;
    green = chroma;
  } else if (hueSegment >= 2 && hueSegment < 3) {
    green = chroma;
    blue = x;
  } else if (hueSegment >= 3 && hueSegment < 4) {
    green = x;
    blue = chroma;
  } else if (hueSegment >= 4 && hueSegment < 5) {
    red = x;
    blue = chroma;
  } else {
    red = chroma;
    blue = x;
  }

  red = Math.round((red + m) * 255);
  green = Math.round((green + m) * 255);
  blue = Math.round((blue + m) * 255);

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
  const whiteness: number = parseFloat(match[2]) / 100;
  const blackness: number = parseFloat(match[3]) / 100;

  const chroma: number = 1 - whiteness - blackness;
  const hueSegment: number = hue / 60;
  const x: number = chroma * (1 - Math.abs((hueSegment % 2) - 1));
  const m: number = 1 - chroma;

  let red: number = 0;
  let green: number = 0;
  let blue: number = 0;

  if (hueSegment >= 0 && hueSegment < 1) {
    red = chroma;
    green = x;
  } else if (hueSegment >= 1 && hueSegment < 2) {
    red = x;
    green = chroma;
  } else if (hueSegment >= 2 && hueSegment < 3) {
    green = chroma;
    blue = x;
  } else if (hueSegment >= 3 && hueSegment < 4) {
    green = x;
    blue = chroma;
  } else if (hueSegment >= 4 && hueSegment < 5) {
    red = x;
    blue = chroma;
  } else {
    red = chroma;
    blue = x;
  }

  red = Math.round((red + whiteness) * 255);
  green = Math.round((green + whiteness) * 255);
  blue = Math.round((blue + whiteness) * 255);

  const rgbString: string = `rgb(${red}, ${green}, ${blue})`;
  return rgbString;
}
