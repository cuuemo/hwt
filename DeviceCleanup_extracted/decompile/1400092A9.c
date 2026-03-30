/*
 * func-name: sub_1400092A9
 * func-address: 0x1400092a9
 * callers: 0x14000929d, 0x140009322, 0x14000932e, 0x14000933a, 0x140009346, 0x140009352, 0x14000935e, 0x14000936a
 * callees: 0x140009c34
 */

__int64 __fastcall sub_1400092A9(__int64 a1, __int64 a2, __int64 a3, __int64 a4)
{
  __int64 v4; // rax
  __int64 (__fastcall *v5)(__int64, __int64, __int64, __int64); // rax

  v5 = (__int64 (__fastcall *)(__int64, __int64, __int64, __int64))sub_140009C34(&SHLWAPI_dll_import_table, v4);
  return v5(a1, a2, a3, a4);
}
