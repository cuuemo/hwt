/*
 * func-name: sub_14000993D
 * func-address: 0x14000993d
 * callers: 0x140009931, 0x1400099b6, 0x1400099c2, 0x1400099ce, 0x1400099da, 0x1400099e6, 0x1400099f2, 0x1400099fe, 0x140009a0a, 0x140009a16, 0x140009a22
 * callees: 0x140009c34
 */

__int64 __fastcall sub_14000993D(__int64 a1, __int64 a2, __int64 a3, __int64 a4)
{
  __int64 v4; // rax
  __int64 (__fastcall *v5)(__int64, __int64, __int64, __int64); // rax

  v5 = (__int64 (__fastcall *)(__int64, __int64, __int64, __int64))sub_140009C34(&GDI32_dll_import_table, v4);
  return v5(a1, a2, a3, a4);
}
