/*
 * func-name: sub_140006378
 * func-address: 0x140006378
 * callers: 0x140006850
 * callees: 0x140006ac0, 0x140007428
 */

LPCSTR *sub_140006378()
{
  LPCSTR *result; // rax
  char **v1; // rbx
  __int64 v2; // rsi
  HMODULE *v3; // rdi
  __int64 v4; // rax

  result = (LPCSTR *)sub_140006AC0();
  v1 = off_14000F6B0;
  if ( (unsigned int)result < 0x600 )
    v1 = off_14000F680;
  v2 = 0;
  if ( *v1 )
  {
    result = (LPCSTR *)v1;
    v3 = &qword_140016F80;
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
