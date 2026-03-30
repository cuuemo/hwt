/*
 * func-name: sub_140002B18
 * func-address: 0x140002b18
 * callers: 0x1400012fc, 0x140003cec
 * callees: none
 */

BOOL sub_140002B18()
{
  BOOL result; // eax
  tagMSG Msg; // [rsp+30h] [rbp-38h] BYREF

  while ( 1 )
  {
    result = PeekMessageW(&Msg, 0, 0, 0, 0);
    if ( !result )
      break;
    if ( GetMessageW(&Msg, 0, 0, 0) )
      DispatchMessageW(&Msg);
  }
  return result;
}
