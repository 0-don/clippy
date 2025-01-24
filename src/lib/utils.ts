// Type definitions
type ClassValue = string | number | boolean | undefined | null | { [key: string]: any } | ClassValue[];

/**
 * Combines class names and handles Tailwind class conflicts
 * @param classLists - Array of class values to be merged
 * @returns Merged class string
 */
export function cn(...classLists: ClassValue[]): string {
  // Helper function to check if a value is an object
  const isObject = (value: any): value is Record<string, any> =>
    value !== null && typeof value === "object" && !Array.isArray(value);

  // clsx functionality
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

  // Simplified tailwind-merge functionality
  function twMerge(classString: string): string {
    const classes = classString.split(" ");
    const classMap = new Map<string, string>();

    const prefixes: readonly string[] = [
      "p-",
      "m-",
      "text-",
      "bg-",
      "border-",
      "rounded-",
      "flex-",
      "grid-",
      "w-",
      "h-",
      "min-",
      "max-",
      "font-",
      "leading-",
    ] as const;

    classes.forEach((cls) => {
      const prefix = prefixes.find((p) => cls.startsWith(p));
      if (prefix) {
        classMap.set(prefix, cls);
      } else {
        classMap.set(cls, cls);
      }
    });

    return Array.from(classMap.values()).join(" ");
  }

  // Combine both functions
  return twMerge(clsx(classLists));
}
