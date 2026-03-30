/*
 * func-name: sub_140007F44
 * func-address: 0x140007f44
 * callers: 0x140003cec, 0x140005518
 * callees: 0x14000179c, 0x140001878
 */

__int64 __fastcall sub_140007F44(const WCHAR *a1)
{
  unsigned int v2; // ebx
  __int64 v3; // rsi
  DWORD TickCount; // edi
  __int64 v5; // rsi
  HGLOBAL v6; // rax
  void *v7; // rdi
  _BYTE *v8; // rax

  v2 = 0;
  v3 = lstrlenW(a1);
  sub_14000179C();
  TickCount = GetTickCount();
  while ( !OpenClipboard(0) )
  {
    Sleep(0x64u);
    if ( GetTickCount() - TickCount >= 0x3E8 )
    {
      GetLastError();
      sub_14000179C();
      return 0;
    }
  }
  EmptyClipboard();
  v5 = 2 * v3 + 2;
  v6 = GlobalAlloc(0x42u, v5);
  v7 = v6;
  if ( v6 )
  {
    v8 = GlobalLock(v6);
    sub_140001878(v8, (__int64)a1, v5);
    GlobalUnlock(v7);
    if ( SetClipboardData(0xDu, v7) )
    {
      v2 = 1;
    }
    else
    {
      GetLastError();
      sub_14000179C();
      GlobalFree(v7);
    }
  }
  CloseClipboard();
  return v2;
}
