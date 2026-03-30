/*
 * func-name: sub_1400054C8
 * func-address: 0x1400054c8
 * callers: 0x140005518
 * callees: none
 */

__int64 __fastcall sub_1400054C8(char a1)
{
  int v1; // ecx
  int v2; // ecx
  int v3; // ecx
  int v4; // ecx

  v1 = (a1 & 0xF0) - 16;
  if ( v1 )
  {
    v2 = v1 - 16;
    if ( !v2 )
      return 32514;
    v3 = v2 - 16;
    if ( !v3 )
      return 32515;
    v4 = v3 - 16;
    if ( !v4 )
      return 32516;
    if ( v4 != 64 )
      return 0;
  }
  return 32513;
}
