#pragma once
#include <functional>
#include <utility>
#include <type_traits>
#include <ranges>
namespace bear::core
{

    template <class T>
    concept IsPointer = std::is_pointer_v<T>;

    template <class T, T Invalid = {} >
    class AutoRelease
    {
    public:

        AutoRelease() :AutoRelease({}, nullptr)
        {
        }

        AutoRelease(T obj, std::function<void(T)> deleter)
            : obj_(obj)
            , deleter_(deleter)
        {

        }

        AutoRelease(const AutoRelease&) = delete;
        auto operator = (const AutoRelease&)->AutoRelease & = delete;


        AutoRelease(AutoRelease&& other) :AutoRelease()
        {
            swap(other);
        }

        auto operator = (AutoRelease&& other) -> AutoRelease&
        {
            AutoRelease new_obj{ std::move(other) };
            swap(new_obj);
            return *this;
        }

        ~AutoRelease()
        {
            if (obj_ != Invalid && deleter_ != nullptr) {
                deleter_(obj_);
            }
        }

        T get() const { return obj_; }

        operator T() const { return obj_; }
        T operator->() const noexcept requires IsPointer<T>
        {
            return obj_;
        }

    private:

        T obj_;
        std::function<void(T)> deleter_;

        auto swap(AutoRelease& other) noexcept -> void
        {
            std::ranges::swap(obj_, other.obj_);
            std::ranges::swap(deleter_, other.deleter_);
        }

    };
}
