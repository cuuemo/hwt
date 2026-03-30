/*
 * func-name: sub_1400083C0
 * func-address: 0x1400083c0
 * callers: 0x140005518
 * callees: none
 */

__int64 sub_1400083C0()
{
  HDC DC; // rdi
  unsigned int DeviceCaps; // ebx

  DC = GetDC(0);
  DeviceCaps = GetDeviceCaps(DC, 10);
  ReleaseDC(0, DC);
  return DeviceCaps;
}
