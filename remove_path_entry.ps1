$git_bin = 'C:/Program Files/Git/usr/bin'
$path = [System.Environment]::GetEnvironmentVariable(
  'PATH',
  'MACHINE'
)

$path = ($path.Split(';') | Where-Object { $_ -ne $git_bin }) -join ';'

[System.Environment]::SetEnvironmentVariable(
  'PATH',
  $path,
  'Machine'
)
