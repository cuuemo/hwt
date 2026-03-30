/*
 * func-name: sub_140009382
 * func-address: 0x140009382
 * callers: 0x140009376, 0x1400093fb, 0x140009407, 0x140009413, 0x14000941f, 0x14000942b, 0x140009437, 0x140009443, 0x14000944f, 0x14000945b, 0x140009467, 0x140009473
 * callees: 0x140009c34
 */

__int64 __fastcall sub_140009382(__int64 a1, __int64 a2, __int64 a3, __int64 a4)
{
  __int64 v4; // rax
  __int64 (__fastcall *v5)(__int64, __int64, __int64, __int64); // rax

  v5 = (__int64 (__fastcall *)(__int64, __int64, __int64, __int64))sub_140009C34(&SETUPAPI_dll_import_table, v4);
  return v5(a1, a2, a3, a4);
}
