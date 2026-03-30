/*
 * func-name: sub_140001878
 * func-address: 0x140001878
 * callers: 0x140005518, 0x1400076c4, 0x140007700, 0x140007f44
 * callees: none
 */

_BYTE *__fastcall sub_140001878(_BYTE *a1, __int64 a2, __int64 a3)
{
  _BYTE *v3; // r9
  __int64 v4; // rdx

  if ( a3 )
  {
    v3 = a1;
    v4 = a2 - (_QWORD)a1;
    do
    {
      *v3 = v3[v4];
      ++v3;
      --a3;
    }
    while ( a3 );
  }
  return a1;
}
