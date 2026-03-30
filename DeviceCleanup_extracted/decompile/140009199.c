/*
 * func-name: sub_140009199
 * func-address: 0x140009199
 * callers: 0x14000918d
 * callees: 0x140009c34
 */

__int64 __fastcall sub_140009199(__int64 a1, __int64 a2, __int64 a3, __int64 a4)
{
  __int64 v4; // rax
  __int64 (__fastcall *v5)(__int64, __int64, __int64, __int64); // rax

  v5 = (__int64 (__fastcall *)(__int64, __int64, __int64, __int64))sub_140009C34(&WINTRUST_dll_import_table, v4);
  return v5(a1, a2, a3, a4);
}
