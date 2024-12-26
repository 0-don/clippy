import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import utc from "dayjs/plugin/utc";

// Import all locales
import "dayjs/locale/ar";
import "dayjs/locale/bn";
import "dayjs/locale/de";
import "dayjs/locale/es";
import "dayjs/locale/fr";
import "dayjs/locale/hi";
import "dayjs/locale/it";
import "dayjs/locale/ja";
import "dayjs/locale/ko";
import "dayjs/locale/nl";
import "dayjs/locale/pl";
import "dayjs/locale/pt";
import "dayjs/locale/ru";
import "dayjs/locale/th";
import "dayjs/locale/tr";
import "dayjs/locale/ur";
import "dayjs/locale/vi";
import "dayjs/locale/zh";

// Extend dayjs with plugins
dayjs.extend(relativeTime);
dayjs.extend(utc);

export default dayjs;
