/*
 * src/controller.cpp
 * tasinput2 - Plugin for creating TAS inputs
 *
 * This file is part of tasinput2.
 *
 * tasinput2 is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * tasinput2 is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with tasinput2.  If not, see <https://www.gnu.org/licenses/>.
 */

#include "controller.hpp"

#include "plugin.h"
#include "tasinput2.h"

#include <sstream>
#include <string>
#include <tuple>

#include <QHBoxLayout>
#include <QLabel>
#include <QVBoxLayout>

/* Directionals */

inline QString ty_format(char ty_indicator, const char *after) {
  std::stringstream ss;
  ss << ty_indicator << after;
  return QString::fromStdString(ss.str());
}

DirectionalPanel::DirectionalPanel(QWidget *parent, DirectionalType ty,
                                   Inputs *inputs)
    : QWidget(parent) {
  this->inputs = inputs;
  this->ty = ty;

  // figure out which char represents the type
  char ty_indicator = ty == DT_C ? 'C' : 'D';

  // up cbox
  QVBoxLayout *layout = new QVBoxLayout(this);
  this->up = new QCheckBox(ty_format(ty_indicator, " Up"));
  layout->addWidget(this->up);

  // left/right cbox
  QWidget *middle = new QWidget();
  layout->addWidget(middle);
  QHBoxLayout *middle_layout = new QHBoxLayout(middle);
  this->left = new QCheckBox(ty_format(ty_indicator, " Left"));
  this->right = new QCheckBox(ty_format(ty_indicator, " Right"));

  middle_layout->addWidget(this->left);
  middle_layout->addWidget(this->right);

  // down cbox
  this->down = new QCheckBox(ty_format(ty_indicator, " Down"));
  layout->addWidget(this->down);

  // connect slots
  connect(this->up, SIGNAL(clicked()), this, SLOT(up_checked()));
  connect(this->down, SIGNAL(clicked()), this, SLOT(down_checked()));
  connect(this->left, SIGNAL(clicked()), this, SLOT(left_checked()));
  connect(this->right, SIGNAL(clicked()), this, SLOT(right_checked()));
}

Directional *DirectionalPanel::get_directional() {
  return this->ty == DT_C ? &this->inputs->c : &this->inputs->d;
}

// click handlers
void DirectionalPanel::up_checked() {
  lock_mutex();
  this->get_directional()->up = this->up->isChecked();
  unlock_mutex();
}

void DirectionalPanel::down_checked() {
  lock_mutex();
  this->get_directional()->down = this->down->isChecked();
  unlock_mutex();
}

void DirectionalPanel::left_checked() {
  lock_mutex();
  this->get_directional()->left = this->left->isChecked();
  unlock_mutex();
}

void DirectionalPanel::right_checked() {
  lock_mutex();
  this->get_directional()->right = this->right->isChecked();
  unlock_mutex();
}

/* Button Panel */

ButtonPanel::ButtonPanel(QWidget *parent, Inputs *inputs) : QWidget(parent) {
  this->inputs = inputs;

  QHBoxLayout *layout = new QHBoxLayout(this);

  this->d = new DirectionalPanel(this, DT_D, inputs);
  layout->addWidget(d);

  this->l = new QCheckBox("L");
  layout->addWidget(this->l);

  // Add some buttons
  QWidget *middle = new QWidget();
  layout->addWidget(middle);
  QVBoxLayout *middle_layout = new QVBoxLayout(middle);

  this->a = new QCheckBox("A");
  this->b = new QCheckBox("B");
  this->z = new QCheckBox("Z");
  this->start = new QCheckBox("Start");

  middle_layout->addWidget(this->z);
  middle_layout->addWidget(this->a);
  middle_layout->addWidget(this->b);
  middle_layout->addWidget(this->start);

  // Add R button and C directional
  this->r = new QCheckBox("R");
  layout->addWidget(this->r);

  this->c = new DirectionalPanel(this, DT_C, inputs);
  layout->addWidget(this->c);

  // connect up clicked signals
  connect(this->a, SIGNAL(clicked()), this, SLOT(a_checked()));
  connect(this->b, SIGNAL(clicked()), this, SLOT(b_checked()));
  connect(this->z, SIGNAL(clicked()), this, SLOT(z_checked()));
  connect(this->l, SIGNAL(clicked()), this, SLOT(l_checked()));
  connect(this->r, SIGNAL(clicked()), this, SLOT(r_checked()));
  connect(this->start, SIGNAL(clicked()), this, SLOT(start_checked()));
}

// Button slots
void ButtonPanel::a_checked() {
  lock_mutex();
  this->inputs->a = this->a->isChecked();
  unlock_mutex();
}

void ButtonPanel::b_checked() {
  lock_mutex();
  this->inputs->b = this->b->isChecked();
  unlock_mutex();
}

void ButtonPanel::z_checked() {
  lock_mutex();
  this->inputs->z = this->z->isChecked();
  unlock_mutex();
}

void ButtonPanel::l_checked() {
  lock_mutex();
  this->inputs->l = this->l->isChecked();
  unlock_mutex();
}

void ButtonPanel::r_checked() {
  lock_mutex();
  this->inputs->r = this->r->isChecked();
  unlock_mutex();
}

void ButtonPanel::start_checked() {
  lock_mutex();
  this->inputs->start = this->start->isChecked();
  unlock_mutex();
}

// Helper function to create a labeled spin box.
std::tuple<QWidget *, QSpinBox *> labeled_spinbox(const char *label, int min,
                                                  int max) {
  // general container
  QWidget *container = new QWidget();
  QHBoxLayout *layout = new QHBoxLayout(container);

  // spinbox
  QSpinBox *spinbox = new QSpinBox();
  spinbox->setRange(min, max);
  layout->addWidget(spinbox);

  // label
  QLabel *text = new QLabel(label);
  layout->addWidget(text);

  return std::tuple(container, spinbox);
}

JoystickPanel::JoystickPanel(QWidget *panel, Inputs *inputs) {
  this->inputs = inputs;

  QHBoxLayout *layout = new QHBoxLayout(this);

  // x component
  std::tuple<QWidget *, QSpinBox *> x = labeled_spinbox("X", -127, 127);
  this->x = std::get<1>(x);
  layout->addWidget(std::get<0>(x));

  // y component
  std::tuple<QWidget *, QSpinBox *> y = labeled_spinbox("Y", -127, 127);
  this->y = std::get<1>(y);
  layout->addWidget(std::get<0>(y));

  // signals
  connect(this->x, qOverload<int>(&QSpinBox::valueChanged), this,
          &JoystickPanel::x_changed);
  connect(this->y, qOverload<int>(&QSpinBox::valueChanged), this,
          &JoystickPanel::y_changed);
}

// signals
void JoystickPanel::x_changed(int val) {
  lock_mutex();
  this->inputs->x = val;
  unlock_mutex();
}

void JoystickPanel::y_changed(int val) {
  lock_mutex();
  this->inputs->y = val;
  unlock_mutex();
}

Controller::Controller(QWidget *parent) : QWidget(parent) {
  this->inputs = new Inputs();

  QVBoxLayout *layout = new QVBoxLayout(this);

  this->js = new JoystickPanel(this, this->inputs);
  this->bs = new ButtonPanel(this, this->inputs);

  layout->addWidget(this->js);
  layout->addWidget(this->bs);
}

Controller::~Controller() { delete this->inputs; }

#include "moc_controller.cpp"
