/*
 * func-name: sub_140009A3A
 * func-address: 0x140009a3a
 * callers: 0x140009a2e, 0x140009ab3, 0x140009abf, 0x140009acb, 0x140009ad7, 0x140009ae3, 0x140009aef, 0x140009afb, 0x140009b07, 0x140009b13, 0x140009b1f, 0x140009b2b, 0x140009b37, 0x140009b43, 0x140009b4f, 0x140009b5b, 0x140009b67, 0x140009b73, 0x140009b7f, 0x140009b8b
 * callees: 0x140009c34
 */

__int64 __fastcall sub_140009A3A(__int64 a1, __int64 a2, __int64 a3, __int64 a4)
{
  __int64 v4; // rax
  __int64 (__fastcall *v5)(__int64, __int64, __int64, __int64); // rax

  v5 = (__int64 (__fastcall *)(__int64, __int64, __int64, __int64))sub_140009C34(&ADVAPI32_dll_import_table, v4);
  return v5(a1, a2, a3, a4);
}
