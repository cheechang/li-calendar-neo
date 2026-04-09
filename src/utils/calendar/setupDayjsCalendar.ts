import dayjs from 'dayjs';
import updateLocale from 'dayjs/plugin/updateLocale';
import 'dayjs/locale/zh-cn';
import 'dayjs/plugin/localeData';

/** 全局使用中文区域，与界面文案一致 */
dayjs.locale('zh-cn');

dayjs.extend(updateLocale);
/** 将周一至周日的展示改为中文全称（与产品表头「一二…日」可并存） */
dayjs.updateLocale('zh-cn', {
  weekdays: ['周日', '周一', '周二', '周三', '周四', '周五', '周六'],
  weekdaysShort: ['周日', '周一', '周二', '周三', '周四', '周五', '周六'],
  weekdaysMin: ['周日', '周一', '周二', '周三', '周四', '周五', '周六'],
});
