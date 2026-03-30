/*
 * func-name: sub_140008400
 * func-address: 0x140008400
 * callers: 0x140005518
 * callees: none
 */

__int64 sub_140008400()
{
  HDC DC; // rdi
  unsigned int DeviceCaps; // ebx

  DC = GetDC(0);
  DeviceCaps = GetDeviceCaps(DC, 8);
  ReleaseDC(0, DC);
  return DeviceCaps;
}
