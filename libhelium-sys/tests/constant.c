// Generated by gir (https://github.com/gtk-rs/gir @ 1201a74cd03e)
// from .. (@ 2e6066273448+)
// from ../gir-files (@ 7ebd4478b4a5)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) HE_ABOUT_WINDOW_LICENSES_APACHEv2);
    PRINT_CONSTANT((gint) HE_ABOUT_WINDOW_LICENSES_GPLv3);
    PRINT_CONSTANT((gint) HE_ABOUT_WINDOW_LICENSES_MIT);
    PRINT_CONSTANT((gint) HE_ABOUT_WINDOW_LICENSES_MPLv2);
    PRINT_CONSTANT((gint) HE_ABOUT_WINDOW_LICENSES_PROPRIETARY);
    PRINT_CONSTANT((gint) HE_ABOUT_WINDOW_LICENSES_UNLICENSE);
    PRINT_CONSTANT((gint) HE_ABOUT_WINDOW_LICENSES_WTFPL);
    PRINT_CONSTANT((gint) HE_BANNER_STYLE_ERROR);
    PRINT_CONSTANT((gint) HE_BANNER_STYLE_INFO);
    PRINT_CONSTANT((gint) HE_BANNER_STYLE_WARNING);
    PRINT_CONSTANT((gint) HE_BOTTOM_BAR_POSITION_LEFT);
    PRINT_CONSTANT((gint) HE_BOTTOM_BAR_POSITION_RIGHT);
    PRINT_CONSTANT((gint) HE_COLORS_BLUE);
    PRINT_CONSTANT((gint) HE_COLORS_BROWN);
    PRINT_CONSTANT((gint) HE_COLORS_DARK);
    PRINT_CONSTANT((gint) HE_COLORS_GREEN);
    PRINT_CONSTANT((gint) HE_COLORS_INDIGO);
    PRINT_CONSTANT((gint) HE_COLORS_LIGHT);
    PRINT_CONSTANT((gint) HE_COLORS_MINT);
    PRINT_CONSTANT((gint) HE_COLORS_NONE);
    PRINT_CONSTANT((gint) HE_COLORS_ORANGE);
    PRINT_CONSTANT((gint) HE_COLORS_PINK);
    PRINT_CONSTANT((gint) HE_COLORS_PURPLE);
    PRINT_CONSTANT((gint) HE_COLORS_RED);
    PRINT_CONSTANT((gint) HE_COLORS_YELLOW);
    PRINT_CONSTANT(HE_COLOR_BLACK);
    PRINT_CONSTANT(HE_COLOR_LAB_CONSTANTS_Kn);
    PRINT_CONSTANT(HE_COLOR_LAB_CONSTANTS_Xn);
    PRINT_CONSTANT(HE_COLOR_LAB_CONSTANTS_Yn);
    PRINT_CONSTANT(HE_COLOR_LAB_CONSTANTS_Zn);
    PRINT_CONSTANT(HE_COLOR_LAB_CONSTANTS_t0);
    PRINT_CONSTANT(HE_COLOR_LAB_CONSTANTS_t1);
    PRINT_CONSTANT(HE_COLOR_LAB_CONSTANTS_t2);
    PRINT_CONSTANT(HE_COLOR_LAB_CONSTANTS_t3);
    PRINT_CONSTANT(HE_COLOR_WHITE);
    PRINT_CONSTANT((gint) HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_BOTTOM_LEFT);
    PRINT_CONSTANT((gint) HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_BOTTOM_RIGHT);
    PRINT_CONSTANT((gint) HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_TOP_LEFT);
    PRINT_CONSTANT((gint) HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_TOP_RIGHT);
    PRINT_CONSTANT((gint) HE_DESKTOP_COLOR_SCHEME_DARK);
    PRINT_CONSTANT((gint) HE_DESKTOP_COLOR_SCHEME_LIGHT);
    PRINT_CONSTANT((gint) HE_DESKTOP_COLOR_SCHEME_NO_PREFERENCE);
    PRINT_CONSTANT((gint) HE_MODIFIER_BADGE_ALIGNMENT_CENTER);
    PRINT_CONSTANT((gint) HE_MODIFIER_BADGE_ALIGNMENT_LEFT);
    PRINT_CONSTANT((gint) HE_MODIFIER_BADGE_ALIGNMENT_RIGHT);
    PRINT_CONSTANT((gint) HE_OVERLAY_BUTTON_ALIGNMENT_CENTER);
    PRINT_CONSTANT((gint) HE_OVERLAY_BUTTON_ALIGNMENT_LEFT);
    PRINT_CONSTANT((gint) HE_OVERLAY_BUTTON_ALIGNMENT_RIGHT);
    PRINT_CONSTANT((gint) HE_OVERLAY_BUTTON_SIZE_LARGE);
    PRINT_CONSTANT((gint) HE_OVERLAY_BUTTON_SIZE_MEDIUM);
    PRINT_CONSTANT((gint) HE_OVERLAY_BUTTON_SIZE_SMALL);
    PRINT_CONSTANT((gint) HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_ALWAYS);
    PRINT_CONSTANT((gint) HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_NEVER);
    PRINT_CONSTANT((gint) HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_SINGLE);
    return 0;
}
