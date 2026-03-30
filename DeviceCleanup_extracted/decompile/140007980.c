/*
 * func-name: sub_140007980
 * func-address: 0x140007980
 * callers: 0x140003584
 * callees: 0x140007268
 */

__int64 __fastcall sub_140007980(const WCHAR *a1, const WCHAR *a2)
{
  int v2; // edi
  int v5; // ebx
  __int64 result; // rax

  v2 = 99999;
  v5 = 99999;
  if ( a1 && sub_140007268(a1, L"COM") )
    v2 = StrToIntW(a1 + 3);
  if ( a2 && sub_140007268(a2, L"COM") )
    v5 = StrToIntW(a2 + 3);
  if ( v2 == v5 )
    return 0;
  result = 0xFFFFFFFFLL;
  if ( v2 > v5 )
    return 1;
  return result;
}
