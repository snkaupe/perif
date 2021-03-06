#!/usr/bin/env python3

# Stolen from https://github.com/cybre/budgie-screenshot-applet/blob/master/data/meson_post_install.py

import os
import subprocess

schemadir = os.path.join(os.environ['MESON_INSTALL_PREFIX'], 'share', 'glib-2.0', 'schemas')

if not os.environ.get('DESTDIR'):
	print('Compiling gsettings schemas...')
	subprocess.call(['glib-compile-schemas', schemadir])
	subprocess.call(['gtk-update-icon-cache', '/usr/share/icons/hicolor'])
