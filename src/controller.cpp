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
#include "tasinput2.hpp"
#include <sstream>
#include <string>

#include <QHBoxLayout>
#include <QVBoxLayout>

inline QString ty_format(char ty_indicator, const char *after) {
    std::stringstream ss;
    ss << ty_indicator << after;
    return QString::fromStdString(ss.str());
}

DirectionalPanel::DirectionalPanel(QWidget *parent, DirectionalType ty, Inputs *inputs) : QWidget(parent) {
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
    get_lock().lock();
    this->get_directional()->up = this->up->isChecked();
    get_lock().unlock();
}

void DirectionalPanel::down_checked() {
    get_lock().lock();
    this->get_directional()->down = this->down->isChecked();
    get_lock().unlock();
}

void DirectionalPanel::left_checked() {
    get_lock().lock();
    this->get_directional()->left = this->left->isChecked();
    get_lock().unlock();
}

void DirectionalPanel::right_checked() {
    get_lock().lock();
    this->get_directional()->right = this->right->isChecked();
    get_lock().unlock();
}

ButtonPanel::ButtonPanel(QWidget *parent, Inputs *inputs) : QWidget(parent) {
    this->a = new QCheckBox("A", this);
    this->inputs = inputs;
}

Controller::Controller(QWidget *parent) : QWidget(parent) {
    this->inputs = new Inputs();
}

Controller::~Controller() {
    delete this->inputs;
}
