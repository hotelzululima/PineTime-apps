/*
 * Copyright (C) 2018 Koen Zandberg <koen@bergzand.net>
 *
 * This file is subject to the terms and conditions of the GNU Lesser
 * General Public License v2.1. See the file LICENSE in the top level
 * directory for more details.
 */

#include <stdint.h>
#include "log.h"
#include "lvgl.h"

static void _screen_menu_pressed(lv_obj_t *obj, lv_event_t event)
{
    switch (event) {
        case LV_EVENT_CLICKED:
            LOG_INFO("button clicked!\n");
            break;
        default:
            break;
    }
}

lv_obj_t *screen_menu_create(void)
{
    lv_obj_t *scr = lv_obj_create(NULL, NULL);

    lv_obj_t * list1 = lv_list_create(scr, NULL);
    lv_obj_set_size(list1, 240, 240);
    lv_obj_align(list1, NULL, LV_ALIGN_CENTER, 0, 0);

    lv_obj_t *list_btn;

    list_btn = lv_list_add_btn(list1, LV_SYMBOL_FILE, "New");
    lv_obj_set_event_cb(list_btn, _screen_menu_pressed);


    list_btn = lv_list_add_btn(list1, LV_SYMBOL_DIRECTORY, "Open");

    list_btn = lv_list_add_btn(list1, LV_SYMBOL_CLOSE, "Delete");

    list_btn = lv_list_add_btn(list1, LV_SYMBOL_EDIT, "Edit");

    list_btn = lv_list_add_btn(list1, LV_SYMBOL_SAVE, "Save");
    (void)list_btn;

    return scr;
}
