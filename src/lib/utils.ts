type ClassValue =
  | string
  | number
  | boolean
  | undefined
  | null
  | { [key: string]: any }
  | ClassValue[];

export function cn(...classLists: ClassValue[]): string {
  return clsx(classLists);
}

function clsx(classes: ClassValue[]): string {
  return classes
    .flat()
    .filter(Boolean)
    .map((cls): string => {
      if (typeof cls === "string") return cls;
      if (isObject(cls)) {
        return Object.entries(cls)
          .filter(([_, value]) => Boolean(value))
          .map(([key]) => key)
          .join(" ");
      }
      return "";
    })
    .join(" ");
}

const isObject = (value: any): value is Record<string, any> =>
  value !== null && typeof value === "object" && !Array.isArray(value);
