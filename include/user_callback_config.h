#pragma once

#include "app_callback.h"
#include "app_default_handlers.h"
#include "app_entry_point.h"
#include "app_prf_types.h"
#include <stdio.h>
#if (BLE_APP_SEC)
#include "app_bond_db.h"
#endif // (BLE_APP_SEC)

extern const struct app_callbacks user_app_callbacks;
extern const struct default_app_operations user_default_app_operations;
extern const struct arch_main_loop_callbacks user_app_main_loop_callbacks;

extern void app_process_catch_rest_cb(ke_msg_id_t const msgid,
                                      void const *param,
                                      ke_task_id_t const dest_id,
                                      ke_task_id_t const src_id);
#if (BLE_APP_SEC)
extern const struct app_bond_db_callbacks user_app_bond_db_callbacks;
#endif
