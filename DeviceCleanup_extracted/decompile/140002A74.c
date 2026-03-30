/*
 * func-name: sub_140002A74
 * func-address: 0x140002a74
 * callers: 0x1400052dc
 * callees: 0x140006ac0, 0x140007428
 */

LPCSTR *sub_140002A74()
{
  LPCSTR *result; // rax
  char **v1; // rbx
  __int64 v2; // rsi
  HMODULE *v3; // rdi
  __int64 v4; // rax

  result = (LPCSTR *)sub_140006AC0();
  v1 = off_14000F540;
  if ( (unsigned int)result < 0x600 )
    v1 = off_14000F4B0;
  v2 = 0;
  if ( *v1 )
  {
    result = (LPCSTR *)v1;
    v3 = &hLibModule;
    do
    {
      v4 = sub_140007428(*result);
      if ( v4 )
        *v3++ = (HMODULE)v4;
      result = (LPCSTR *)&v1[++v2];
    }
    while ( *result );
  }
  return result;
}
