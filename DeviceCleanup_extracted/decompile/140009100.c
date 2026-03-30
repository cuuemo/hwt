/*
 * func-name: sub_140009100
 * func-address: 0x140009100
 * callers: 0x140003cec
 * callees: 0x140001000, 0x140001058, 0x1400010fc, 0x140001110, 0x140001128, 0x140001130, 0x140001898, 0x1400018d4
 */

__int64 __fastcall sub_140009100(int a1)
{
  unsigned int v3; // esi
  LPVOID v4; // rax
  void *v5; // rbx

  if ( !a1 )
    return 0;
  v3 = 0;
  v4 = sub_140001898(0x18u);
  if ( v4 )
    v5 = (void *)sub_140001000((__int64)v4);
  else
    v5 = 0;
  sub_140001130((__int64)v5);
  if ( (unsigned int)sub_1400010FC((__int64)v5, a1 - 1) )
  {
    sub_140001110((__int64)v5, a1 - 1, 0);
    v3 = sub_140001058((__int64)v5);
  }
  if ( v5 )
  {
    sub_140001128((__int64)v5);
    sub_1400018D4(v5);
  }
  return v3;
}
