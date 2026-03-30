/*
 * func-name: sub_140002AE4
 * func-address: 0x140002ae4
 * callers: 0x1400052dc
 * callees: none
 */

HMODULE sub_140002AE4()
{
  HMODULE result; // rax
  __int64 v1; // rbx

  result = hLibModule[0];
  v1 = 0;
  while ( result )
  {
    FreeLibrary(result);
    result = hLibModule[++v1];
  }
  return result;
}
