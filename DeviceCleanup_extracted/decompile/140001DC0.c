/*
 * func-name: sub_140001DC0
 * func-address: 0x140001dc0
 * callers: 0x140002b64, 0x140007b74
 * callees: 0x1400018e8
 */

WCHAR *sub_140001DC0()
{
  DWORD LastError; // eax
  const char *v1; // rax

  LastError = GetLastError();
  v1 = sub_1400018E8(LastError);
  MultiByteToWideChar(0, 0, v1, -1, &WideCharStr, 512);
  return &WideCharStr;
}
