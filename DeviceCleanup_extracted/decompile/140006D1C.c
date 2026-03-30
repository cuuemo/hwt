/*
 * func-name: sub_140006D1C
 * func-address: 0x140006d1c
 * callers: 0x1400012fc
 * callees: 0x140001854
 */

HRESULT __fastcall sub_140006D1C(__int64 a1, CLSID *a2)
{
  if ( *(_WORD *)(a1 + 76) )
    return (unsigned int)sub_140001854(a2, 0, 0x10u);
  else
    return CLSIDFromString((LPCOLESTR)a1, a2);
}
