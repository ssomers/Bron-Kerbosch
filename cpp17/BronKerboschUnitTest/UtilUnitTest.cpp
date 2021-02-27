#include "pch.h"

#include "BronKerbosch/Util.h"

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace BronKerbosch {
    TEST_CLASS(UtilUnitTest) {
       public:
        template <typename VertexSet>
        void Util_pop_arbitray() {
            auto one = VertexSet{1};
            auto two = VertexSet{1, 2};
            Assert::AreEqual(1, Util::pop_arbitrary(one));
            Assert::IsTrue(one.empty());
            auto x = Util::pop_arbitrary(two);
            auto y = Util::pop_arbitrary(two);
            Assert::AreEqual(1, std::min(x, y));
            Assert::AreEqual(2, std::max(x, y));
        }

        template <typename VertexSet>
        void Util_are_disjoint() {
            auto const empty = VertexSet{};
            auto const one = VertexSet{1};
            auto const two = VertexSet{1, 2};
            auto const six = VertexSet{0, 1, 2, 3, 4, 5};
            Assert::IsTrue(Util::are_disjoint(empty, one));
            Assert::IsTrue(Util::are_disjoint(one, empty));
            Assert::IsTrue(Util::are_disjoint(empty, two));
            Assert::IsTrue(Util::are_disjoint(two, empty));
            Assert::IsTrue(Util::are_disjoint(empty, six));
            Assert::IsTrue(Util::are_disjoint(six, empty));
            Assert::IsFalse(Util::are_disjoint(one, two));
            Assert::IsFalse(Util::are_disjoint(two, one));
            Assert::IsFalse(Util::are_disjoint(one, six));
            Assert::IsFalse(Util::are_disjoint(six, one));
            Assert::IsFalse(Util::are_disjoint(two, six));
            Assert::IsFalse(Util::are_disjoint(six, two));
            Assert::IsFalse(Util::are_disjoint(one, one));
            Assert::IsFalse(Util::are_disjoint(two, two));
            Assert::IsFalse(Util::are_disjoint(six, six));
        }

        template <typename VertexSet>
        void Util_intersection() {
            auto const empty = VertexSet{};
            auto const one = VertexSet{1};
            auto const two = VertexSet{1, 2};
            auto const six = VertexSet{0, 1, 2, 3, 4, 5};
            Assert::IsTrue(Util::intersection(empty, one) == empty);
            Assert::IsTrue(Util::intersection(one, empty) == empty);
            Assert::IsTrue(Util::intersection(empty, two) == empty);
            Assert::IsTrue(Util::intersection(two, empty) == empty);
            Assert::IsTrue(Util::intersection(empty, six) == empty);
            Assert::IsTrue(Util::intersection(six, empty) == empty);
            Assert::IsTrue(Util::intersection(one, two) == one);
            Assert::IsTrue(Util::intersection(two, one) == one);
            Assert::IsTrue(Util::intersection(one, six) == one);
            Assert::IsTrue(Util::intersection(six, one) == one);
            Assert::IsTrue(Util::intersection(two, six) == two);
            Assert::IsTrue(Util::intersection(six, two) == two);
            Assert::IsTrue(Util::intersection(one, one) == one);
            Assert::IsTrue(Util::intersection(two, two) == two);
            Assert::IsTrue(Util::intersection(six, six) == six);
        }

        template <typename VertexSet>
        void Util_intersection_size() {
            auto const empty = VertexSet{};
            auto const one = VertexSet{1};
            auto const two = VertexSet{1, 2};
            auto const six = VertexSet{0, 1, 2, 3, 4, 5};
            Assert::IsTrue(Util::intersection_size(empty, one) == 0);
            Assert::IsTrue(Util::intersection_size(one, empty) == 0);
            Assert::IsTrue(Util::intersection_size(empty, two) == 0);
            Assert::IsTrue(Util::intersection_size(two, empty) == 0);
            Assert::IsTrue(Util::intersection_size(empty, six) == 0);
            Assert::IsTrue(Util::intersection_size(six, empty) == 0);
            Assert::IsTrue(Util::intersection_size(one, two) == 1);
            Assert::IsTrue(Util::intersection_size(two, one) == 1);
            Assert::IsTrue(Util::intersection_size(one, six) == 1);
            Assert::IsTrue(Util::intersection_size(six, one) == 1);
            Assert::IsTrue(Util::intersection_size(two, six) == 2);
            Assert::IsTrue(Util::intersection_size(six, two) == 2);
            Assert::IsTrue(Util::intersection_size(one, one) == 1);
            Assert::IsTrue(Util::intersection_size(two, two) == 2);
            Assert::IsTrue(Util::intersection_size(six, six) == 6);
        }

        template <typename VertexSet>
        void Util_difference() {
            auto const empty = VertexSet{};
            auto const one = VertexSet{1};
            auto const two = VertexSet{1, 2};
            auto const six = VertexSet{0, 1, 2, 3, 4, 5};
            Assert::IsTrue(Util::difference(empty, one) == empty);
            Assert::IsTrue(Util::difference(empty, two) == empty);
            Assert::IsTrue(Util::difference(empty, six) == empty);
            Assert::IsTrue(Util::difference(one, one) == empty);
            Assert::IsTrue(Util::difference(one, two) == empty);
            Assert::IsTrue(Util::difference(one, six) == empty);
            Assert::IsTrue(Util::difference(two, two) == empty);
            Assert::IsTrue(Util::difference(two, six) == empty);
            Assert::IsTrue(Util::difference(six, six) == empty);
            Assert::IsTrue(Util::difference(one, empty) == one);
            Assert::IsTrue(Util::difference(two, empty) == two);
            Assert::IsTrue(Util::difference(six, empty) == six);
            Assert::IsTrue(Util::difference(two, one) == VertexSet{2});
            Assert::IsTrue(Util::difference(six, one) == VertexSet{0, 2, 3, 4, 5});
            Assert::IsTrue(Util::difference(six, two) == VertexSet{0, 3, 4, 5});
        }

        TEST_METHOD(Util_pop_arbitray_set) {
            Util_pop_arbitray<std::set<int>>();
        }
        TEST_METHOD(Util_pop_arbitray_ordered_vector) {
            Util_pop_arbitray<ordered_vector<int>>();
        }
        TEST_METHOD(Util_pop_arbitray_unordered_set) {
            Util_pop_arbitray<std::unordered_set<int>>();
        }

        TEST_METHOD(Util_are_disjoint_set) {
            Util_are_disjoint<std::set<int>>();
        }
        TEST_METHOD(Util_are_disjoint_ordered_vector) {
            Util_are_disjoint<ordered_vector<int>>();
        }
        TEST_METHOD(Util_are_disjoint_unordered_set) {
            Util_are_disjoint<std::unordered_set<int>>();
        }

        TEST_METHOD(Util_intersection_size_set) {
            Util_intersection_size<std::set<int>>();
        }
        TEST_METHOD(Util_intersection_size_ordered_vector) {
            Util_intersection_size<ordered_vector<int>>();
        }
        TEST_METHOD(Util_intersection_size_unordered_set) {
            Util_intersection_size<std::unordered_set<int>>();
        }

        TEST_METHOD(Util_intersection_set) {
            Util_intersection<std::set<int>>();
        }
        TEST_METHOD(Util_intersection_ordered_vector) {
            Util_intersection<ordered_vector<int>>();
        }
        TEST_METHOD(Util_intersection_unordered_set) {
            Util_intersection<std::unordered_set<int>>();
        }

        TEST_METHOD(Util_difference_set) {
            Util_difference<std::set<int>>();
        }
        TEST_METHOD(Util_difference_ordered_vector) {
            Util_difference<ordered_vector<int>>();
        }
        TEST_METHOD(Util_difference_unordered_set) {
            Util_difference<std::unordered_set<int>>();
        }
    };
}
