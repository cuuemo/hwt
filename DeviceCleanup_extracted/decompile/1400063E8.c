/*
 * func-name: sub_1400063E8
 * func-address: 0x1400063e8
 * callers: 0x140006850
 * callees: none
 */

HMODULE sub_1400063E8()
{
  HMODULE result; // rax
  __int64 v1; // rbx

  result = qword_140016F80;
  v1 = 0;
  while ( result )
  {
    FreeLibrary(result);
    ++v1;
    result = *(&qword_140016F80 + v1);
  }
  return result;
}
