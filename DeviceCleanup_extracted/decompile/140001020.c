/*
 * func-name: sub_140001020
 * func-address: 0x140001020
 * callers: 0x140001128, 0x140001130
 * callees: 0x1400018d4
 */

__int64 __fastcall sub_140001020(__int64 a1)
{
  __int64 result; // rax

  *(_DWORD *)(a1 + 16) = 0;
  if ( *(_QWORD *)(a1 + 8) )
  {
    result = sub_1400018D4();
    *(_QWORD *)(a1 + 8) = 0;
  }
  if ( *(_QWORD *)a1 )
  {
    result = sub_1400018D4();
    *(_QWORD *)a1 = 0;
  }
  return result;
}
