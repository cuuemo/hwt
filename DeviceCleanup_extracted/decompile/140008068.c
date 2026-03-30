/*
 * func-name: sub_140008068
 * func-address: 0x140008068
 * callers: 0x140003bcc, 0x140005518
 * callees: none
 */

int __fastcall sub_140008068(HWND hWnd, HWND *a2)
{
  const char *v3; // r14
  HMODULE LibraryA; // rax
  HMODULE v6; // rbp
  HRESULT (__stdcall *SetWindowTheme)(HWND, LPCWSTR, LPCWSTR); // r13
  __int64 v8; // rdi
  HWND *v9; // rbx

  v3 = " ";
  if ( dword_14000F458 )
    v3 = 0;
  LibraryA = LoadLibraryA("UxTheme.dll");
  v6 = LibraryA;
  if ( LibraryA )
  {
    SetWindowTheme = (HRESULT (__stdcall *)(HWND, LPCWSTR, LPCWSTR))GetProcAddress(LibraryA, "SetWindowTheme");
    if ( SetWindowTheme )
    {
      v8 = 0;
      if ( *a2 )
      {
        v9 = a2;
        do
        {
          ((void (__fastcall *)(HWND, const char *, const char *))SetWindowTheme)(*v9, v3, v3);
          if ( IsWindowVisible(*v9) )
          {
            ShowWindow(*v9, 0);
            ShowWindow(*v9, 5);
          }
          v9 = &a2[++v8];
        }
        while ( *v9 );
      }
    }
    FreeLibrary(v6);
    InvalidateRect(hWnd, 0, 1);
    LODWORD(LibraryA) = UpdateWindow(hWnd);
  }
  return (int)LibraryA;
}
