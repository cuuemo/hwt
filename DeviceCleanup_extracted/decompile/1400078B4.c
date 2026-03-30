/*
 * func-name: sub_1400078B4
 * func-address: 0x1400078b4
 * callers: 0x1400036d4
 * callees: 0x140001840
 */

WCHAR *__fastcall sub_1400078B4(unsigned __int64 a1)
{
  unsigned int v1; // eax
  int v2; // eax
  const WCHAR *v3; // rdx
  const wchar_t *v4; // r9

  v1 = sub_140001840(a1, 0x3E8u, 0);
  if ( v1 <= 0x1E13380 )
  {
    if ( v1 <= 0x15180 )
    {
      if ( v1 <= 0xE10 )
      {
        if ( v1 <= 0x3C )
        {
          wsprintfW(&word_140016FE0, L"%i Seconds", v1);
          return &word_140016FE0;
        }
        v2 = sub_140001840(v1, 0x3Cu, 0);
        v3 = L"%i Minute%s";
      }
      else
      {
        v2 = sub_140001840(v1, 0xE10u, 0);
        v3 = L"%i Hour%s";
      }
    }
    else
    {
      v2 = sub_140001840(v1, 0x15180u, 0);
      v3 = L"%i Day%s";
    }
  }
  else
  {
    v2 = sub_140001840(v1, 0x278D00u, 0);
    v3 = L"%i Month%s";
  }
  v4 = &Default;
  if ( v2 > 1 )
    v4 = L"s";
  wsprintfW(&word_140016FE0, v3, (unsigned int)v2, v4);
  return &word_140016FE0;
}
