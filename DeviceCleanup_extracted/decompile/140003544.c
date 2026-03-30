/*
 * func-name: sub_140003544
 * func-address: 0x140003544
 * callers: 0x140003584
 * callees: 0x1400034d8
 */

__int64 __fastcall sub_140003544(int a1, int a2)
{
  __int64 v3; // rdi
  __int64 v4; // r11
  __int64 result; // rax

  v3 = sub_1400034D8(a1);
  v4 = sub_1400034D8(a2);
  if ( v3 == v4 )
    return 0;
  result = 0xFFFFFFFFLL;
  if ( v3 > v4 )
    return 1;
  return result;
}
