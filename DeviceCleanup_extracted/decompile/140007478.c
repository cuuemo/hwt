/*
 * func-name: sub_140007478
 * func-address: 0x140007478
 * callers: 0x1400047fc
 * callees: 0x1400073d8
 */

HMODULE sub_140007478()
{
  HMODULE result; // rax
  HMODULE v1; // rdi
  BOOL (__stdcall *IsThemeActive)(); // rax
  unsigned int v3; // ebx

  result = sub_1400073D8(L"UxTheme.dll");
  v1 = result;
  if ( result )
  {
    IsThemeActive = (BOOL (__stdcall *)())GetProcAddress(result, "IsThemeActive");
    v3 = IsThemeActive();
    FreeLibrary(v1);
    return (HMODULE)v3;
  }
  return result;
}
