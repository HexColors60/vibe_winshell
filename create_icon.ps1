Add-Type -AssemblyName System.Drawing

try {
    $size = 256
    $bmp = New-Object System.Drawing.Bitmap $size, $size
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias

    # Transparent background
    $g.Clear([System.Drawing.Color]::Transparent)

    # Draw Circles
    $pen1 = New-Object System.Drawing.Pen ([System.Drawing.Color]::DeepSkyBlue), 12
    $pen2 = New-Object System.Drawing.Pen ([System.Drawing.Color]::MediumPurple), 10

    $rect1 = New-Object System.Drawing.Rectangle 16, 16, ($size - 32), ($size - 32)
    $rect2 = New-Object System.Drawing.Rectangle 40, 40, ($size - 80), ($size - 80)

    $g.DrawEllipse($pen1, $rect1)
    $g.DrawEllipse($pen2, $rect2)

    # Draw "W"
    # Use simple 2-arg constructor to avoid overload ambiguity
    $font = New-Object System.Drawing.Font "Arial", 110.0
    
    $brush = [System.Drawing.Brushes]::White
    $sf = New-Object System.Drawing.StringFormat
    $sf.Alignment = [System.Drawing.StringAlignment]::Center
    $sf.LineAlignment = [System.Drawing.StringAlignment]::Center

    $g.DrawString("W", $font, $brush, ($size / 2), ($size / 2) + 15, $sf)

    $g.Dispose()

    # Save as PNG
    $bmp.Save("winshell.png", [System.Drawing.Imaging.ImageFormat]::Png)

    # Save as ICO
    $hIcon = $bmp.GetHicon()
    $icon = [System.Drawing.Icon]::FromHandle($hIcon)

    $fs = New-Object System.IO.FileStream "winshell.ico", "Create"
    $icon.Save($fs)
    $fs.Close()

    $bmp.Dispose()
    $icon.Dispose()
    Write-Host "Icon (ICO and PNG) created successfully."
}
catch {
    Write-Error $_
}
