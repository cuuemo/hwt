/*
 * func-name: sub_140001D84
 * func-address: 0x140001d84
 * callers: 0x140002b64, 0x140002e90
 * callees: none
 */

CHAR *__fastcall sub_140001D84(unsigned int a1)
{
  if ( a1 <= 0x3B )
    return off_14000F270[a1];
  wsprintfA(byte_14000FBA8, "CR_0x%X", a1);
  return byte_14000FBA8;
}
