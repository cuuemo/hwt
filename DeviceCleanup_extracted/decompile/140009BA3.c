/*
 * func-name: sub_140009BA3
 * func-address: 0x140009ba3
 * callers: 0x140009b97, 0x140009c1c, 0x140009c28
 * callees: 0x140009c34
 */

__int64 __fastcall sub_140009BA3(__int64 a1, __int64 a2, __int64 a3, __int64 a4)
{
  __int64 v4; // rax
  __int64 (__fastcall *v5)(__int64, __int64, __int64, __int64); // rax

  v5 = (__int64 (__fastcall *)(__int64, __int64, __int64, __int64))sub_140009C34(&ole32_dll_import_table, v4);
  return v5(a1, a2, a3, a4);
}
